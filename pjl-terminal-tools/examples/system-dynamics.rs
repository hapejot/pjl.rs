use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;

use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

#[derive(Debug, Clone)]
struct Node {
    id: String,
    name: String,
    node_type: NodeType,
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    Stock,     // Bestand
    Flow,      // Fluss
    Variable,  // Variable
    Connector, // Verbindung
}

#[derive(Debug, Clone)]
struct Connection {
    id: String,
    source: String,
    target: String,
    connection_type: ConnectionType,
}

#[derive(Debug, Clone)]
enum ConnectionType {
    MaterialFlow, // Materialfluss
    Information,  // Informationsfluss
    Positive,     // Positive Rückkopplung
    Negative,     // Negative Rückkopplung
}

struct SystemDynamicsModel {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    next_id: u32,
}

impl SystemDynamicsModel {
    fn new() -> Self {
        SystemDynamicsModel {
            nodes: Vec::new(),
            connections: Vec::new(),
            next_id: 1,
        }
    }

    fn generate_id(&mut self, prefix: &str) -> String {
        let id = format!("{}{}", prefix, self.next_id);
        self.next_id += 1;
        id
    }

    fn add_stock(&mut self, name: &str, x: i32, y: i32) -> String {
        let id = self.generate_id("stock");
        self.nodes.push(Node {
            id: id.clone(),
            name: name.to_string(),
            node_type: NodeType::Stock,
            x,
            y,
        });
        id
    }

    fn add_flow(&mut self, name: &str, x: i32, y: i32) -> String {
        let id = self.generate_id("flow");
        self.nodes.push(Node {
            id: id.clone(),
            name: name.to_string(),
            node_type: NodeType::Flow,
            x,
            y,
        });
        id
    }

    fn add_variable(&mut self, name: &str, x: i32, y: i32) -> String {
        let id = self.generate_id("var");
        self.nodes.push(Node {
            id: id.clone(),
            name: name.to_string(),
            node_type: NodeType::Variable,
            x,
            y,
        });
        id
    }

    fn add_connector(&mut self, name: &str, x: i32, y: i32) -> String {
        let id = self.generate_id("conn");
        self.nodes.push(Node {
            id: id.clone(),
            name: name.to_string(),
            node_type: NodeType::Connector,
            x,
            y,
        });
        id
    }

    fn connect(&mut self, source: &str, target: &str, conn_type: ConnectionType) {
        let id = self.generate_id("edge");
        self.connections.push(Connection {
            id,
            source: source.to_string(),
            target: target.to_string(),
            connection_type: conn_type,
        });
    }

    fn export_to_drawio(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        // XML-Header und draw.io Basis-Struktur
        let header = r#"<mxfile host="app.diagrams.net" modified="2023-04-26T12:00:00.000Z" agent="Mozilla/5.0" version="21.0.2" etag="abc123" type="device">
  <diagram name="System Dynamics Model" id="system-dynamics">
    <mxGraphModel dx="1422" dy="798" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="1169" pageHeight="827" math="0" shadow="0">
      <root>
        <mxCell id="0" />
        <mxCell id="1" parent="0" />
"#;
        file.write_all(header.as_bytes())?;

        // Knoten exportieren
        for node in &self.nodes {
            let style = match node.node_type {
                NodeType::Stock => "rounded=0;whiteSpace=wrap;html=1;fillColor=#dae8fc;strokeColor=#6c8ebf;",
                NodeType::Flow => "shape=rhombus;whiteSpace=wrap;html=1;fillColor=#fff2cc;strokeColor=#d6b656;",
                NodeType::Variable => "ellipse;whiteSpace=wrap;html=1;fillColor=#d5e8d4;strokeColor=#82b366;",
                NodeType::Connector => "shape=parallelogram;perimeter=parallelogramPerimeter;whiteSpace=wrap;html=1;fixedSize=1;fillColor=#f8cecc;strokeColor=#b85450;",
            };

            let width = match node.node_type {
                NodeType::Stock => 120,
                NodeType::Flow => 80,
                NodeType::Variable => 100,
                NodeType::Connector => 90,
            };

            let height = match node.node_type {
                NodeType::Stock => 60,
                NodeType::Flow => 80,
                NodeType::Variable => 60,
                NodeType::Connector => 40,
            };

            let node_xml = format!(
                r#"        <mxCell id="{}" value="{}" style="{}" vertex="1" parent="1">
          <mxGeometry x="{}" y="{}" width="{}" height="{}" as="geometry" />
        </mxCell>
"#,
                node.id, node.name, style, node.x, node.y, width, height
            );
            file.write_all(node_xml.as_bytes())?;
        }

        // Verbindungen exportieren
        for conn in &self.connections {
            let style = match conn.connection_type {
                ConnectionType::MaterialFlow => "edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;strokeWidth=2;",
                ConnectionType::Information => "edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;dashed=1;",
                ConnectionType::Positive => "edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;endArrow=classic;endFill=1;",
                ConnectionType::Negative => "edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;endArrow=classic;endFill=0;",
            };

            let label = match conn.connection_type {
                ConnectionType::Positive => "+",
                ConnectionType::Negative => "-",
                _ => "",
            };

            let conn_xml = format!(
                r#"        <mxCell id="{}" value="{}" style="{}" edge="1" parent="1" source="{}" target="{}">
          <mxGeometry relative="1" as="geometry" />
        </mxCell>
"#,
                conn.id, label, style, conn.source, conn.target
            );
            file.write_all(conn_xml.as_bytes())?;
        }

        // XML-Footer
        let footer = r#"      </root>
    </mxGraphModel>
  </diagram>
</mxfile>"#;
        file.write_all(footer.as_bytes())?;

        Ok(())
    }
}

// Struktur für unser UI
struct App {
    model: SystemDynamicsModel,
    node_map: HashMap<String, String>,
    menu_state: MenuState,
    input_buffer: String,
    messages: Vec<String>,
    selected_option: usize,
    input_prompt: String,
    exported_file: Option<String>,
}

enum MenuState {
    MainMenu,
    AddNode(NodeType),
    // AddConnection,
    SelectSource,
    SelectTarget,
    SelectConnectionType,
    Export,
}

impl App {
    fn new() -> Self {
        App {
            model: SystemDynamicsModel::new(),
            node_map: HashMap::new(),
            menu_state: MenuState::MainMenu,
            input_buffer: String::new(),
            messages: Vec::new(),
            selected_option: 0,
            input_prompt: String::new(),
            exported_file: None,
        }
    }

    fn add_message(&mut self, message: &str) {
        self.messages.push(message.to_string());
        if self.messages.len() > 5 {
            self.messages.remove(0);
        }
    }

    fn handle_key_event(&mut self, key: KeyCode) -> bool {
        match self.menu_state {
            MenuState::MainMenu => match key {
                KeyCode::Char('q') | KeyCode::Esc => return false,
                KeyCode::Down => {
                    self.selected_option = (self.selected_option + 1) % 7;
                }
                KeyCode::Up => {
                    if self.selected_option > 0 {
                        self.selected_option -= 1;
                    } else {
                        self.selected_option = 6;
                    }
                }
                KeyCode::Enter => {
                    match self.selected_option {
                        0 => self.menu_state = MenuState::AddNode(NodeType::Stock),
                        1 => self.menu_state = MenuState::AddNode(NodeType::Flow),
                        2 => self.menu_state = MenuState::AddNode(NodeType::Variable),
                        3 => self.menu_state = MenuState::AddNode(NodeType::Connector),
                        4 => {
                            if self.model.nodes.len() < 2 {
                                self.add_message("Mindestens 2 Knoten benötigt!");
                            } else {
                                self.menu_state = MenuState::SelectSource;
                                self.selected_option = 0;
                            }
                        }
                        5 => self.menu_state = MenuState::Export,
                        6 => return false,
                        _ => {}
                    }
                    self.input_buffer.clear();
                }
                _ => {}
            },
            MenuState::AddNode(_) => {
                match key {
                    KeyCode::Esc => {
                        self.menu_state = MenuState::MainMenu;
                        self.selected_option = 0;
                    }
                    KeyCode::Enter => {
                        if !self.input_buffer.is_empty() {
                            let name = self.input_buffer.clone();
                            self.input_buffer.clear();

                            // Einfache Auto-Positionierung
                            let node_count = self.model.nodes.len() as i32;
                            let x = 100 + (node_count % 5) * 150;
                            let y = 100 + (node_count / 5) * 100;

                            let node_type = match self.menu_state {
                                MenuState::AddNode(ref t) => t.clone(),
                                _ => unreachable!(),
                            };

                            let id = match node_type {
                                NodeType::Stock => self.model.add_stock(&name, x, y),
                                NodeType::Flow => self.model.add_flow(&name, x, y),
                                NodeType::Variable => self.model.add_variable(&name, x, y),
                                NodeType::Connector => self.model.add_connector(&name, x, y),
                            };

                            self.node_map.insert(name.clone(), id);

                            let type_name = match node_type {
                                NodeType::Stock => "Stock",
                                NodeType::Flow => "Flow",
                                NodeType::Variable => "Variable",
                                NodeType::Connector => "Connector",
                            };

                            self.add_message(&format!("{} '{}' hinzugefügt", type_name, name));
                            self.menu_state = MenuState::MainMenu;
                            self.selected_option = 0;
                        }
                    }
                    KeyCode::Backspace => {
                        self.input_buffer.pop();
                    }
                    KeyCode::Char(c) => {
                        self.input_buffer.push(c);
                    }
                    _ => {}
                }
            }
            MenuState::SelectSource | MenuState::SelectTarget => {
                let nodes: Vec<&String> = self.node_map.keys().collect();

                match key {
                    KeyCode::Esc => {
                        self.menu_state = MenuState::MainMenu;
                        self.selected_option = 0;
                    }
                    KeyCode::Down => {
                        if !nodes.is_empty() {
                            self.selected_option = (self.selected_option + 1) % nodes.len();
                        }
                    }
                    KeyCode::Up => {
                        if !nodes.is_empty() {
                            if self.selected_option > 0 {
                                self.selected_option -= 1;
                            } else {
                                self.selected_option = nodes.len() - 1;
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if !nodes.is_empty() {
                            let selected_node = nodes[self.selected_option].clone();

                            match self.menu_state {
                                MenuState::SelectSource => {
                                    self.input_buffer = selected_node;
                                    self.menu_state = MenuState::SelectTarget;
                                }
                                MenuState::SelectTarget => {
                                    let source = self.input_buffer.clone();
                                    let target = selected_node;

                                    // Überprüfen, ob Quelle und Ziel gleich sind
                                    if source == target {
                                        self.add_message(
                                            "Quelle und Ziel dürfen nicht gleich sein!",
                                        );
                                    } else {
                                        self.input_buffer = source;
                                        self.menu_state = MenuState::SelectConnectionType;
                                        self.input_prompt = format!(
                                            "Verbindung: {} -> {}",
                                            self.input_buffer, target
                                        );
                                        self.selected_option = 0;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            MenuState::SelectConnectionType => match key {
                KeyCode::Esc => {
                    self.menu_state = MenuState::MainMenu;
                    self.selected_option = 0;
                }
                KeyCode::Down => {
                    self.selected_option = (self.selected_option + 1) % 4;
                }
                KeyCode::Up => {
                    if self.selected_option > 0 {
                        self.selected_option -= 1;
                    } else {
                        self.selected_option = 3;
                    }
                }
                KeyCode::Enter => {
                    let nodes: Vec<&String> = self.node_map.keys().collect();
                    let source = self.input_buffer.clone();
                    let target = nodes[self.selected_option].clone();

                    let conn_type = match self.selected_option {
                        0 => ConnectionType::MaterialFlow,
                        1 => ConnectionType::Information,
                        2 => ConnectionType::Positive,
                        3 => ConnectionType::Negative,
                        _ => ConnectionType::Information,
                    };

                    let source_id = self.node_map.get(&source).unwrap();
                    let target_id = self.node_map.get(&target).unwrap();

                    self.model.connect(source_id, target_id, conn_type);

                    let conn_name = match self.selected_option {
                        0 => "Materialfluss",
                        1 => "Informationsfluss",
                        2 => "Positive Rückkopplung",
                        3 => "Negative Rückkopplung",
                        _ => "Verbindung",
                    };

                    self.add_message(&format!("{} erstellt: {} -> {}", conn_name, source, target));
                    self.menu_state = MenuState::MainMenu;
                    self.selected_option = 0;
                }
                _ => {}
            },
            MenuState::Export => match key {
                KeyCode::Esc => {
                    self.menu_state = MenuState::MainMenu;
                    self.selected_option = 0;
                }
                KeyCode::Enter => {
                    if !self.input_buffer.is_empty() {
                        let filename = if !self.input_buffer.ends_with(".drawio") {
                            format!("{}.drawio", self.input_buffer)
                        } else {
                            self.input_buffer.clone()
                        };

                        match self.model.export_to_drawio(&filename) {
                            Ok(_) => {
                                self.add_message(&format!("Modell nach '{}' exportiert", filename));
                                self.exported_file = Some(filename);
                            }
                            Err(e) => {
                                self.add_message(&format!("Fehler beim Export: {}", e));
                            }
                        }

                        self.menu_state = MenuState::MainMenu;
                        self.selected_option = 0;
                        self.input_buffer.clear();
                    }
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                }
                _ => {}
            }
        }

        true
    }

    fn render(&self) -> std::io::Result<()> {
        let mut stdout = stdout();

        // Terminal leeren
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),                    
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::Blue),
            Print("System Dynamics Modellierung".to_string()),
            ResetColor
        )?;

        // Hauptmenü
        execute!(stdout, cursor::MoveTo(0, 2))?;

        match self.menu_state {
            MenuState::MainMenu => {
                // Statusinfos oben anzeigen
                execute!(
                    stdout,
                    cursor::MoveTo(0, 1),
                    Print(format!(
                        "Knoten: {} | Verbindungen: {} ",
                        self.model.nodes.len(),
                        self.model.connections.len()
                    ))
                )?;

                let options = [
                    "Stock hinzufügen",
                    "Flow hinzufügen",
                    "Variable hinzufügen",
                    "Connector hinzufügen",
                    "Verbindung erstellen",
                    "Modell exportieren",
                    "Beenden",
                ];

                for (i, option) in options.iter().enumerate() {
                    if i == self.selected_option {
                        execute!(
                            stdout,
                            cursor::MoveTo(2, (i as u16) + 3),
                            SetBackgroundColor(Color::DarkGrey),
                            Print(format!("➤ {}", option)),
                            ResetColor
                        )?;
                    } else {
                        execute!(
                            stdout,
                            cursor::MoveTo(2, (i as u16) + 3),
                            Print(format!("  {}", option))
                        )?;
                    }
                }

                // Hinweise
                execute!(
                    stdout,
                    cursor::MoveTo(0, 12),
                    Print("Navigationshilfe:"),
                    cursor::MoveTo(0, 13),
                    Print("▲/▼: Auswahl bewegen | Enter: Bestätigen | ESC: Zurück/Beenden")
                )?;
            }
            MenuState::AddNode(ref node_type) => {
                let type_name = match node_type {
                    NodeType::Stock => "Stock (Bestand)",
                    NodeType::Flow => "Flow (Fluss)",
                    NodeType::Variable => "Variable",
                    NodeType::Connector => "Connector (Verbindung)",
                };

                execute!(
                    stdout,
                    cursor::MoveTo(0, 3),
                    Print(format!("{} hinzufügen", type_name)),
                    cursor::MoveTo(0, 5),
                    Print("Name eingeben: "),
                    Print(&self.input_buffer),
                    cursor::MoveTo(0, 7),
                    Print("Enter: Bestätigen | ESC: Abbrechen")
                )?;
            }
            MenuState::SelectSource | MenuState::SelectTarget => {
                let title = match self.menu_state {
                    MenuState::SelectSource => "Quellknoten auswählen",
                    MenuState::SelectTarget => "Zielknoten auswählen",
                    _ => "",
                };

                execute!(stdout, cursor::MoveTo(0, 3), Print(title))?;

                let nodes: Vec<&String> = self.node_map.keys().collect();

                for (i, node) in nodes.iter().enumerate() {
                    let node_info = {
                        let id = self.node_map.get(*node).unwrap();
                        let node_entry = self.model.nodes.iter().find(|n| n.id == *id).unwrap();
                        let node_type = match node_entry.node_type {
                            NodeType::Stock => "Stock",
                            NodeType::Flow => "Flow",
                            NodeType::Variable => "Var",
                            NodeType::Connector => "Conn",
                        };
                        format!("{} ({})", node, node_type)
                    };

                    if i == self.selected_option {
                        execute!(
                            stdout,
                            cursor::MoveTo(2, (i as u16) + 5),
                            SetBackgroundColor(Color::DarkGrey),
                            Print(format!("➤ {}", node_info)),
                            ResetColor
                        )?;
                    } else {
                        execute!(
                            stdout,
                            cursor::MoveTo(2, (i as u16) + 5),
                            Print(format!("  {}", node_info))
                        )?;
                    }
                }

                execute!(
                    stdout,
                    cursor::MoveTo(0, 18),
                    Print("▲/▼: Auswahl bewegen | Enter: Bestätigen | ESC: Zurück")
                )?;
            }
            MenuState::SelectConnectionType => {
                execute!(
                    stdout,
                    cursor::MoveTo(0, 3),
                    Print("Verbindungstyp auswählen"),
                    cursor::MoveTo(0, 4),
                    Print(&self.input_prompt)
                )?;

                let options = [
                    "Materialfluss",
                    "Informationsfluss",
                    "Positive Rückkopplung",
                    "Negative Rückkopplung",
                ];

                for (i, option) in options.iter().enumerate() {
                    if i == self.selected_option {
                        execute!(
                            stdout,
                            cursor::MoveTo(2, (i as u16) + 6),
                            SetBackgroundColor(Color::DarkGrey),
                            Print(format!("➤ {}", option)),
                            ResetColor
                        )?;
                    } else {
                        execute!(
                            stdout,
                            cursor::MoveTo(2, (i as u16) + 6),
                            Print(format!("  {}", option))
                        )?;
                    }
                }

                execute!(
                    stdout,
                    cursor::MoveTo(0, 12),
                    Print("▲/▼: Auswahl bewegen | Enter: Bestätigen | ESC: Zurück")
                )?;
            }
            MenuState::Export => {
                execute!(
                    stdout,
                    cursor::MoveTo(0, 3),
                    Print("Dateinamen für den Export eingeben:"),
                    cursor::MoveTo(0, 5),
                    Print("Dateiname: "),
                    Print(&self.input_buffer),
                    cursor::MoveTo(0, 7),
                    Print("Enter: Bestätigen | ESC: Abbrechen")
                )?;
            }
        }

        // Meldungen anzeigen
        execute!(
            stdout,
            cursor::MoveTo(0, 20),
            SetForegroundColor(Color::Yellow)
        )?;
        for (i, message) in self.messages.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, 20 + i as u16), Print(message))?;
        }
        execute!(stdout, ResetColor)?;

        // Wenn eine Datei exportiert wurde, zeige Erfolgsinfo an
        if let Some(ref filename) = self.exported_file {
            execute!(
                stdout,
                cursor::MoveTo(40, 3),
                SetForegroundColor(Color::Green),
                Print(format!("Erfolgreicher Export nach:")),
                cursor::MoveTo(40, 4),
                Print(format!("{}", filename)),
                cursor::MoveTo(40, 5),
                Print("Datei kann jetzt in draw.io geöffnet werden!"),
                ResetColor
            )?;
        }

        stdout.flush()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    // Terminal in Raw-Mode und Alternate Screen wechseln
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut app = App::new();
    app.add_message("Willkommen! Erstellen Sie Ihr System Dynamics Modell.");

    // Event-Loop
    loop {
        app.render()?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if !app.handle_key_event(key.code) {
                    break;
                }
            }
        }
    }

    // Terminal wiederherstellen
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;

    Ok(())
}
