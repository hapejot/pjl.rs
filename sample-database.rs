fn setup_database(cfg: &Config) -> Database {
    let db = DatabaseBuilder::new().build();
    let mut model = DataModel::new("actor")
        .table(
            Table::new("path")
                .field("id", true, "string")
                .field("name", false, "string"),
        )
        .table(
            Table::new("file")
                .field("id", true, "string")
                .field("pathid", false, "string")
                .field("size", false, "int")
                .field("name", false, "string"),
        )
        .table(
            Table::new("actor")
                .field("id", true, "string")
                .field("name", false, "string")
                .field("bio", false, "string")
                .field("birthPlace", false, "string")
                .field("birthday", false, "string")
                .field("gender", false, "string")
                .field("height", false, "string")
                .field("lastSceneReleaseDate", false, "string")
                .field("measurements", false, "string")
                .field("rank", false, "string")
                .field("weight", false, "string"),
        )
        .table(
            Table::new("tag")
                .field("id", true, "string")
                .field("name", false, "string")
                .field("category", false, "string")
                .field("categoryOrder", false, "string"),
        )
        .table(
            Table::new("actor_tag")
                .field("actorid", true, "string")
                .field("tagid", true, "string"),
        )
        .table(
            Table::new("scene_tag")
                .field("sceneid", true, "string")
                .field("tagid", true, "string"),
        )
        .table(
            Table::new("actor_scene")
                .field("sceneid", true, "string")
                .field("actorid", true, "string"),
        )
        .table(
            Table::new("path_scene")
                .field("pathid", true, "string")
                .field("sceneid", false, "string")
                .field("ignore", false, "boolean"),
        )
        .table(
            Table::new("scene")
                .field("id", true, "string")
                .field("title", false, "string")
                .field("description", false, "string")
                .field("video_1080p_sizeBytes", false, "string")
                .field("video_1080p_download", false, "string")
                .field("video_1080p_view", false, "string")
                .field("video_1080p_format", false, "string")
                .field("video_length", false, "int"),
            // .field("videos", false, "string")
        );

    let mut meta = Meta::new();
    let relation = meta.define_relation(ManyMany("scene_tag".into()), "scene", "tags", "tag");
    meta.map_field(relation.as_str(), "scene.id", "sceneid");
    meta.map_field(relation.as_str(), "tag.id", "tagid");

    let relation = meta.define_relation(ManyMany("actor_tag".into()), "actor", "tags", "tag");
    meta.map_field(relation.as_str(), "actor.id", "actorid");
    meta.map_field(relation.as_str(), "tag.id", "tagid");

    let relation =
        meta.define_relation(ManyMany("actor_scene".into()), "scene", "actors", "actorid");
    meta.map_field(relation.as_str(), "scene.id", "sceneid");
    meta.map_field(relation.as_str(), "actor.id", "actorid");

    let _rel = meta.define_relation(RelationKind::One, "file", "pathid", "Path");
    meta.map_field(_rel.as_str(), "pathid", "id");

    let _ = meta.define_relation(
        Embedded {
            prefix: Some("video".into()),
        },
        "scene",
        "videos",
        "videoid",
    );

    let _ = meta.define_relation(Embedded { prefix: None }, "Videos", "full", "videoid");

    let _ = meta.define_relation(Embedded { prefix: None }, "Video", "files", "to");

    let _ = meta.define_relation(
        Embedded {
            prefix: Some("1080p".into()),
        },
        "Files",
        "1080p",
        "to",
    );

    let _ = meta.define_relation(Embedded { prefix: None }, "Media", "urls", "to");

    model.set_meta(meta);

    db.connect(Some(cfg.database.as_str()));

    db.activate_structure(model);
    db
}
