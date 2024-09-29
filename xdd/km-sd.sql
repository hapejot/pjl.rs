CREATE TABLE thead (
    tid char(100) NOT NULL)
;
CREATE TABLE vbak (
    vbeln char(10) NOT NULL)
;
CREATE TABLE vbap (
    posnr numc(6) NOT NULL,
    vbeln char(10) NOT NULL)
;
CREATE TABLE zbc_proc_header (
    gjahr numc(4) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    primary key (gjahr,proc_code,proc_counter) )
;
CREATE TABLE zsd_4pl_check (
    auart string NOT NULL,
    pstyv string NOT NULL,
    primary key (auart,pstyv) )
;
CREATE TABLE zsd_action_sets (
    active string NOT NULL,
    auart char(4) NOT NULL,
    matkl char(9) NOT NULL,
    pstyv_bom char(4) NOT NULL,
    pstyv_item_from char(4) NOT NULL,
    pstyv_item_to char(4) NOT NULL,
    pstyv_material char(18) NOT NULL,
    pstyv_uemat char(18) NOT NULL,
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,matkl,pstyv_bom,pstyv_item_from,pstyv_item_to,pstyv_material,pstyv_uemat,spart,vkorg,vtweg) )
;
CREATE TABLE zsd_agrkm_action (
    auart string NOT NULL,
    begda dats(8) NOT NULL,
    endda dats(8),
    event char(32),
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_agrkm_admin_data (
    admin_mail char(241),
    admin_name char(40),
    vat_id char(20))
;
CREATE TABLE zsd_agrkm_cont (
    matnr char(18) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (matnr,vkorg) )
;
CREATE TABLE zsd_agrkm_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    reference_key char(70),
    reference_type char(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_agrkm_mat_view (
    arktx char(40),
    line_number int4(10),
    matnr char(18),
    menge int4(10))
;
CREATE TABLE zsd_agrkm_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_agrkm_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_agrkm_st_t (
    bezei char(50) NOT NULL,
    method char(30) NOT NULL,
    spras string NOT NULL,
    step string NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_agrkm_st_v (
    bezei char(50),
    method char(30),
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_agrkm_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_append_komp (
    zzataxk1 char(1),
    zzbewtar string,
    zzktgrm string,
    zzmatnr_g string,
    zzmenge_g numc(7),
    zzprod1 char(2),
    zzpstyv char(4),
    zzsta char(17),
    zzstawn8 char(8),
    zzvstel string,
    zzwadty curr(13))
;
CREATE TABLE zsd_batausw (
    datab dats(8),
    datbi dats(8),
    kbetr curr(11),
    kmein unit(3),
    konwa cuky(5),
    kpein dec(5),
    kschl char(4),
    maktx char(40),
    matnr char(18),
    zzqcodegrp char(8))
;
CREATE TABLE zsd_bes_face_bk (
    ad_city1 char(40),
    ad_name1 char(40),
    ad_street char(60),
    ad_tlnmbr1 char(30),
    contact char(40),
    zzbes_f_alias char(40),
    zzbes_f_code5 char(20) NOT NULL,
    primary key (zzbes_f_code5) )
;
CREATE TABLE zsd_bes_face_cod (
    adrnr char(10),
    code char(20) NOT NULL,
    erdat dats(8),
    ernam char(12),
    typ char(2) NOT NULL,
    primary key (code,typ) )
;
CREATE TABLE zsd_bes_face_con (
    adm_ref_n char(35),
    aedat dats(8),
    aenam char(12),
    code1 char(20),
    code2 char(20),
    code3 char(20),
    code4 char(20),
    code5 string,
    dire_code char(20),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    pccn char(30),
    vbeln string NOT NULL,
    zz_bstkd char(35),
    primary key (vbeln) )
;
CREATE TABLE zsd_bes_face_cv (
    adm_ref_n char(35),
    code1 char(20),
    code2 char(20),
    code3 char(20),
    code4 char(20),
    dire_code char(20),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    pccn char(30),
    vbeln string NOT NULL,
    zz_bstkd char(35),
    primary key (vbeln) )
;
CREATE TABLE zsd_bes_face_typ (
    name char(30),
    typ char(2) NOT NULL,
    primary key (typ) )
;
CREATE TABLE zsd_bill_blocck (
    auart char(4) NOT NULL,
    faksk char(2),
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    vsbed char(2) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,spart,vkorg,vsbed,vtweg) )
;
CREATE TABLE zsd_billplan (
    fkmng quan(13),
    fplnr char(10) NOT NULL,
    fpltr numc(6) NOT NULL,
    posnr numc(6) NOT NULL,
    vbeln string,
    primary key (fplnr,fpltr,posnr) )
;
CREATE TABLE zsd_bit_freight (
    netwr curr(15) NOT NULL,
    vkgrp char(3) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    zamount dec(5),
    primary key (netwr,vkgrp,vkorg,vtweg) )
;
CREATE TABLE zsd_block_area (
    area char(3) NOT NULL,
    vkorg char(4),
    primary key (area) )
;
CREATE TABLE zsd_block_areat (
    area string NOT NULL,
    descr char(40),
    spras lang(1) NOT NULL,
    primary key (area,spras) )
;
CREATE TABLE zsd_block_config (
    area string NOT NULL,
    block char(2) NOT NULL,
    class string,
    field string NOT NULL,
    subject char(50),
    text char(70),
    primary key (area,block,field) )
;
CREATE TABLE zsd_block_mail (
    area string NOT NULL,
    block string NOT NULL,
    email char(241) NOT NULL,
    field string NOT NULL,
    primary key (area,block,email,field) )
;
CREATE TABLE zsd_bmeng (
    auart char(4) NOT NULL,
    ekgrp char(3),
    pstyv char(4) NOT NULL,
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,pstyv,spart,vkorg,vtweg) )
;
CREATE TABLE zsd_bom_copy_x001_to_x000 (
    kommentar char(80),
    matnr char(18),
    stlal char(2),
    stlal2 char(2),
    stlan char(1),
    stlan2 char(1),
    stlnr char(8),
    stlnr2 char(8),
    werks char(4),
    werks2 char(4))
;
CREATE TABLE zsd_bom_explode (
    alignment int4(10),
    chosen string,
    class int4(10),
    disabled string,
    editable string,
    font int4(10),
    hidden string,
    idnrk char(18),
    idx int4(10),
    ignoreimag string,
    item_name char(12),
    length int4(10),
    length_pix string,
    node_key char(12),
    postp char(1),
    stufe dec(2),
    style int4(10),
    t_image char(46),
    text char(40),
    togg_right string,
    txtisqinfo string,
    uchar char(1),
    usebgcolor string,
    zzcolap char(1))
;
CREATE TABLE zsd_bukbakbapmvk (
    abgru string,
    arktx char(40),
    auart string,
    augru string,
    bstnk char(20),
    cmgst string,
    erdat dats(8),
    ernam char(12),
    knumv char(10),
    kunnr string,
    lfgsk char(1),
    lfstk char(1),
    lsstk char(1),
    matkl string,
    matnr string,
    posnr string NOT NULL,
    prat6 char(1),
    prdha string,
    prodh string,
    pstyv string,
    submi char(10),
    uvall char(1),
    vbeln string NOT NULL,
    vdatu dats(8),
    vkbur string,
    vkgrp string,
    vkorg string,
    vtweg string,
    waerk string,
    zzprodarea string,
    primary key (posnr,vbeln) )
;
CREATE TABLE zsd_c_ordertext (
    auart string NOT NULL,
    tdid string NOT NULL,
    textgroup string NOT NULL,
    vkorg string NOT NULL,
    primary key (auart,tdid,vkorg) )
;
CREATE TABLE zsd_c_ordertextp (
    auart string NOT NULL,
    tdid string NOT NULL,
    textgroup string NOT NULL,
    vkorg string NOT NULL,
    primary key (auart,tdid,vkorg) )
;
CREATE TABLE zsd_c_ordertxtg (
    description char(40) NOT NULL,
    textgroup char(20) NOT NULL,
    primary key (textgroup) )
;
CREATE TABLE zsd_c_ordertxtgf (
    extradata string NOT NULL,
    lfdnr char(3) NOT NULL,
    show_if_empty char(1) NOT NULL,
    textgroup string NOT NULL,
    primary key (lfdnr,textgroup) )
;
CREATE TABLE zsd_cagrkm_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    reference_key char(70),
    reference_type char(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_cagrkm_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_cagrkm_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ccore1_docs (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    ref_data char(30),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ccore1_mess (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ccore1_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    banfn char(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ccore2_docs (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    ref_data char(30),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ccore2_mess (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ccore2_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    banfn char(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ccore3_docs (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ccore3_mess (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ccore3_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    banfn char(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ccore4_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10),
    vbeln_vl char(10),
    primary key (doc_counter,gjahr,proc_code,proc_counter,step) )
;
CREATE TABLE zsd_ccore4_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10),
    vbeln_vl char(10),
    primary key (doc_counter,erdat,erzet,gjahr,proc_code,proc_counter,step) )
;
CREATE TABLE zsd_ccore4_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10),
    vbeln_vl char(10),
    primary key (gjahr,proc_code,proc_counter,step) )
;
CREATE TABLE zsd_ce001_action (
    auart char(4) NOT NULL,
    spart char(2) NOT NULL,
    step char(4) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_cequis_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    vbeln_vl char(10) NOT NULL,
    primary key (doc_counter,gjahr,proc_code,proc_counter,step,vbeln,vbeln_vl) )
;
CREATE TABLE zsd_cequis_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    vbeln_vl char(10) NOT NULL,
    primary key (doc_counter,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln,vbeln_vl) )
;
CREATE TABLE zsd_cequis_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    vbeln_vl char(10) NOT NULL,
    primary key (gjahr,proc_code,proc_counter,step,vbeln,vbeln_vl) )
;
CREATE TABLE zsd_change_eqbs_hard (
    set_efre string,
    set_ekun string,
    set_elag string,
    set_elie string,
    stat_efre char(5),
    stat_ekun char(5),
    stat_elag char(5),
    stat_elie char(5),
    txt04_efre char(4),
    txt04_ekun char(4),
    txt04_elag char(4),
    txt04_elie char(4))
;
CREATE TABLE zsd_clickprice_alv (
    aedat dats(8),
    codegruppe string,
    datab dats(8),
    datbi dats(8),
    erdat dats(8),
    ernam char(12),
    gruppe char(1),
    kbetr dec(11),
    kbetr_fh dec(11),
    kmein string,
    konwa cuky(5),
    kstbm quan(15),
    kstbm_min quan(15),
    kurztext char(40),
    matnr string,
    optional char(1),
    posnr string,
    vbeln string,
    zzpool char(1),
    zzrental dec(5))
;
CREATE TABLE zsd_commission_param_cust (
    currency cuky(5),
    dynnr_zvoe char(4),
    ppom_orgid numc(8),
    price_text_id char(9),
    price_type string,
    prrel string,
    use_order_flags char(1),
    use_provseg string,
    version char(10))
;
CREATE TABLE zsd_commission_parameters (
    clsna string,
    currency cuky(5),
    datab dats(8),
    datbi dats(8),
    descr char(80),
    dynnr_zvoe char(4),
    ppom_orgid numc(8),
    prbeln char(10),
    price_text_id char(9),
    price_type string,
    prrel string,
    spart string,
    tvers char(2),
    use_order_flags char(1),
    use_provseg string,
    vbeln char(10),
    version char(10),
    vkorg string,
    vtweg string)
;
CREATE TABLE zsd_commission_sbal (
    provbeln char(10),
    vorgang char(10))
;
CREATE TABLE zsd_cont_pers (
    addr_comp char(10),
    addrnumber char(10),
    city1 char(40),
    country char(3),
    equnr char(18) NOT NULL,
    fax_number char(30),
    iloan char(12),
    ktokd char(4),
    kunde_ap char(12) NOT NULL,
    kunde_we char(10) NOT NULL,
    langu lang(1),
    name1 char(40),
    name2 char(40),
    name3 char(40),
    objnr char(22) NOT NULL,
    parvw_ap char(2) NOT NULL,
    parvw_we char(2) NOT NULL,
    pers_name char(35),
    post_code1 char(10),
    prsnr char(10),
    spart char(2),
    street char(60),
    tel_number char(30),
    transpzone char(10),
    vkbur char(4),
    vkorg char(4),
    vtweg char(2),
    primary key (equnr,kunde_ap,kunde_we,objnr,parvw_ap,parvw_we) )
;
CREATE TABLE zsd_contr_vbrk (
    click dec(11),
    codgr char(8),
    eqktx char(40),
    equnr char(18),
    erdat dats(8),
    fbuda dats(8),
    fkart char(4),
    fkdat dats(8),
    fkimg quan(13),
    fksto_k char(1),
    freikg char(10),
    freikm quan(15),
    kunag_k char(10),
    kunrg char(10),
    kurztext char(40),
    matnr char(18),
    mwsbp curr(13),
    name1 char(30),
    name_re char(40),
    netwr_pos curr(15),
    posnr numc(6),
    posnv numc(6),
    prsdt dats(8),
    sfakn_k char(10),
    vbegdat dats(8),
    vbeln char(10),
    vbelv char(10),
    venddat dats(8),
    vgbel char(10),
    vgpos numc(6),
    vkbur char(4),
    vkgrp char(3),
    vkorg_k char(4),
    vlcod char(4),
    vreli char(10),
    zaehl_alt char(10),
    zaehl_diff char(10),
    zaehl_neu char(10),
    zterm_k char(4),
    zvba curr(11),
    zvbe curr(11),
    zvbr curr(11),
    zvbs curr(11),
    zzdatab dats(8),
    zzdatbi dats(8))
;
CREATE TABLE zsd_core1_acti_v (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    bezei char(50),
    ekgrp char(3) NOT NULL,
    endda dats(8),
    event string,
    proc_mode string,
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,ekgrp,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_core1_action (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    ekgrp char(3) NOT NULL,
    endda dats(8),
    event string,
    proc_mode string,
    seq_step char(4),
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,ekgrp,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_core1_activ (
    active string,
    ekgrp char(3) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (ekgrp,vkorg) )
;
CREATE TABLE zsd_core1_actv2 (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    ekgrp char(3) NOT NULL,
    endda dats(8),
    event string,
    proc_mode string,
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,ekgrp,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_core1_docs (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    ref_data char(30),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_core1_mess (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_core1_msg_v (
    add_data char(1300) NOT NULL,
    auart string NOT NULL,
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    erdat_va dats(8) NOT NULL,
    ernam char(12) NOT NULL,
    ernam_va char(12) NOT NULL,
    erzet tims(6) NOT NULL,
    erzet_va tims(6) NOT NULL,
    gbstk char(1) NOT NULL,
    global_proc_id char(32) NOT NULL,
    last_step_count int4(10) NOT NULL,
    lock_count numc(10) NOT NULL,
    locked string NOT NULL,
    msgid char(20) NOT NULL,
    msgno numc(3) NOT NULL,
    msgv1 char(50) NOT NULL,
    msgv2 char(50) NOT NULL,
    msgv3 char(50) NOT NULL,
    msgv4 char(50) NOT NULL,
    mtype char(1) NOT NULL,
    noc_code string NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    spart string NOT NULL,
    step char(4) NOT NULL,
    vbeln string NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    primary key (add_data,auart,banfn,doc_counter,ekgrp,erdat,erdat_va,ernam,ernam_va,erzet,erzet_va,gbstk,global_proc_id,last_step_count,lock_count,locked,msgid,msgno,msgv1,msgv2,msgv3,msgv4,mtype,noc_code,proc_code,proc_counter,spart,step,vbeln,vkorg,vtweg) )
;
CREATE TABLE zsd_core1_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_core1_st_t (
    bezei char(50) NOT NULL,
    spras string NOT NULL,
    step string NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_core1_st_v (
    bezei char(50),
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_core1_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    banfn char(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_core1_steps_dialog (
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    banfn char(10),
    bezei char(50),
    doc_counter int4(10),
    ekgrp char(3),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    event char(32),
    gjahr numc(4),
    light char(4),
    mark char(1),
    msgli char(220),
    mtype char(1),
    objky char(70),
    objtp char(10),
    proc_code char(5),
    proc_counter char(32),
    proc_order int4(10),
    step char(4),
    vbeln char(10),
    vicon char(4))
;
CREATE TABLE zsd_core1_steps_s (
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    banfn char(10),
    ekgrp char(3),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4),
    mtype char(1),
    proc_code char(5),
    proc_counter char(32),
    proc_order int4(10),
    step char(4),
    vbeln char(10))
;
CREATE TABLE zsd_core2_acti_v (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    bezei char(50),
    ekgrp char(3) NOT NULL,
    endda dats(8),
    event string,
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,ekgrp,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_core2_action (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    ekgrp char(3) NOT NULL,
    endda dats(8),
    event string,
    seq_step char(4),
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,ekgrp,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_core2_actv2 (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    ekgrp char(3) NOT NULL,
    endda dats(8),
    event string,
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,ekgrp,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_core2_docs (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    ref_data char(30),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_core2_mess (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_core2_msg_v (
    add_data char(1300) NOT NULL,
    auart string NOT NULL,
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    erdat_va dats(8) NOT NULL,
    ernam char(12) NOT NULL,
    ernam_va char(12) NOT NULL,
    erzet tims(6) NOT NULL,
    erzet_va tims(6) NOT NULL,
    gbstk char(1) NOT NULL,
    global_proc_id char(32) NOT NULL,
    last_step_count int4(10) NOT NULL,
    lock_count numc(10) NOT NULL,
    locked string NOT NULL,
    msgid char(20) NOT NULL,
    msgno numc(3) NOT NULL,
    msgv1 char(50) NOT NULL,
    msgv2 char(50) NOT NULL,
    msgv3 char(50) NOT NULL,
    msgv4 char(50) NOT NULL,
    mtype char(1) NOT NULL,
    noc_code string NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    spart string NOT NULL,
    step char(4) NOT NULL,
    vbeln string NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    primary key (add_data,auart,banfn,doc_counter,ekgrp,erdat,erdat_va,ernam,ernam_va,erzet,erzet_va,gbstk,global_proc_id,last_step_count,lock_count,locked,msgid,msgno,msgv1,msgv2,msgv3,msgv4,mtype,noc_code,proc_code,proc_counter,spart,step,vbeln,vkorg,vtweg) )
;
CREATE TABLE zsd_core2_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_core2_st_t (
    bezei char(50) NOT NULL,
    spras string NOT NULL,
    step string NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_core2_st_v (
    bezei char(50),
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_core2_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    banfn char(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_core3_action (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    ekgrp char(3) NOT NULL,
    endda dats(8),
    event char(32),
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,ekgrp,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_core3_bom (
    vkorg string NOT NULL,
    werks_bom string NOT NULL,
    primary key (vkorg) )
;
CREATE TABLE zsd_core3_docs (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_core3_mess (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_core3_msg_v (
    add_data char(1300),
    auart char(4),
    banfn char(10),
    doc_counter int4(10),
    ekgrp char(3),
    erdat dats(8),
    erdat_va dats(8),
    ernam char(12),
    ernam_va char(12),
    erzet tims(6),
    erzet_va tims(6),
    gbstk char(1),
    global_proc_id char(32),
    last_step_count int4(10),
    lock_count numc(10),
    locked string,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1),
    noc_code string,
    proc_code char(5),
    proc_counter char(32),
    spart char(2),
    step char(4),
    vbeln char(10),
    vkorg char(4),
    vtweg char(2))
;
CREATE TABLE zsd_core3_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_core3_st_t (
    bezei char(50) NOT NULL,
    spras string NOT NULL,
    step string NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_core3_st_v (
    bezei char(50),
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_core3_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    banfn char(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_core4_action (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    ekgrp char(3) NOT NULL,
    endda dats(8),
    event char(32),
    spart char(2) NOT NULL,
    step char(4) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,ekgrp,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_core4_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10),
    vbeln_vl char(10),
    primary key (doc_counter,gjahr,proc_code,proc_counter,step) )
;
CREATE TABLE zsd_core4_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10),
    vbeln_vl char(10),
    primary key (doc_counter,erdat,erzet,gjahr,proc_code,proc_counter,step) )
;
CREATE TABLE zsd_core4_msg_v (
    add_data char(1300),
    auart char(4),
    doc_counter int4(10),
    erdat dats(8),
    erdat_va dats(8),
    ernam char(12),
    ernam_va char(12),
    erzet tims(6),
    erzet_va tims(6),
    gbstk char(1),
    global_proc_id char(32),
    last_step_count int4(10),
    lock_count numc(10),
    locked string,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1),
    noc_code string,
    proc_code char(5),
    proc_counter char(32),
    spart char(2),
    step char(4),
    vbeln char(10),
    vbeln_vl char(10),
    vkorg char(4),
    vtweg char(2))
;
CREATE TABLE zsd_core4_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_core4_st_t (
    bezei char(50) NOT NULL,
    spras lang(1) NOT NULL,
    step char(4) NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_core4_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10),
    vbeln_vl char(10),
    primary key (gjahr,proc_code,proc_counter,step) )
;
CREATE TABLE zsd_core4_steps_dialog (
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    bezei char(50),
    doc_counter int4(10),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    event char(32),
    gjahr numc(4),
    light char(4),
    mark char(1),
    msgli char(220),
    mtype char(1),
    objky char(70),
    objtp char(10),
    proc_code char(5),
    proc_counter char(32),
    proc_order int4(10),
    step char(4),
    vbeln char(10),
    vbeln_vl char(10),
    vicon char(4))
;
CREATE TABLE zsd_core4_steps_s (
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4),
    mtype char(1),
    proc_code char(5),
    proc_counter char(32),
    proc_order int4(10),
    step char(4),
    vbeln char(10),
    vbeln_vl char(10))
;
CREATE TABLE zsd_core_action_s (
    auart char(4),
    begda dats(8),
    bezei char(50),
    ekgrp char(3),
    endda dats(8),
    event string,
    proc_code char(5),
    proc_mode string,
    spart char(2),
    step char(4),
    vkorg char(4),
    vtweg char(2))
;
CREATE TABLE zsd_core_ana_row (
    aspect char(70),
    cnt int4(10),
    erdat char(8),
    id raw(16),
    msg char(300),
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    step char(100),
    step_desc char(100),
    swo_objid char(90),
    swo_objtyp char(10))
;
CREATE TABLE zsd_core_analytics_dlg_s (
    ucomm char(70),
    xl_split string)
;
CREATE TABLE zsd_core_analytics_node (
    arg1 char(40),
    arg2 char(40),
    arg3 char(40),
    arg4 char(40),
    arg5 char(40),
    arg6 char(40),
    arg7 char(40),
    arg8 char(40),
    arg9 char(40),
    id raw(16),
    node objectref)
;
CREATE TABLE zsd_core_api_s (
    add_data char(1300),
    auart char(4),
    banfn char(10),
    cnt int4(10),
    doc_counter int4(10),
    ekgrp char(3),
    erdat dats(8),
    erdat_va dats(8),
    ernam char(12),
    ernam_va char(12),
    erzet tims(6),
    erzet_va tims(6),
    gbstk char(1),
    global_proc_id char(32),
    last_step_count int4(10),
    lock_count numc(10),
    locked string,
    msgid char(20),
    msgno numc(3),
    msgtxt char(220),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1),
    noc_code string,
    proc_code char(5),
    proc_counter char(32),
    spart char(2),
    step char(4),
    vbeln char(10),
    vkorg char(4),
    vtweg char(2))
;
CREATE TABLE zsd_core_appl (
    appl string NOT NULL,
    ucomm char(70) NOT NULL,
    primary key (appl,ucomm) )
;
CREATE TABLE zsd_core_aspect (
    activated char(1),
    class char(30),
    ucomm char(70) NOT NULL,
    primary key (ucomm) )
;
CREATE TABLE zsd_core_auart (
    auart string NOT NULL,
    create_auart char(4) NOT NULL,
    create_delblock char(2) NOT NULL,
    create_route char(6) NOT NULL,
    ekgrp string NOT NULL,
    spart string NOT NULL,
    vkorg string NOT NULL,
    vsbed char(2) NOT NULL,
    vtweg string NOT NULL,
    primary key (auart,ekgrp,spart,vkorg,vsbed,vtweg) )
;
CREATE TABLE zsd_core_auart2 (
    auart string NOT NULL,
    create_auart char(4) NOT NULL,
    flag_overtake_preconf char(1) NOT NULL,
    flag_preconf char(1) NOT NULL,
    ship_cond char(2) NOT NULL,
    spart string NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    werks char(4) NOT NULL,
    primary key (auart,flag_preconf,spart,vkorg,vtweg,werks) )
;
CREATE TABLE zsd_core_banfmis (
    banfn char(10),
    proc_banfn char(10),
    proc_counter char(32) NOT NULL,
    vbeln char(10),
    primary key (proc_counter) )
;
CREATE TABLE zsd_core_batch (
    batch_po char(1),
    ekorg string NOT NULL,
    primary key (ekorg) )
;
CREATE TABLE zsd_core_bomhdr (
    bom_hdr_material char(18) NOT NULL,
    pstyv string NOT NULL,
    primary key (pstyv) )
;
CREATE TABLE zsd_core_c2_devs (
    dev char(1),
    ret01_sel char(1),
    sync1_sel char(1),
    uname char(12) NOT NULL,
    primary key (uname) )
;
CREATE TABLE zsd_core_c2_fun (
    description char(200),
    func char(50) NOT NULL,
    primary key (func) )
;
CREATE TABLE zsd_core_c2_grp (
    description char(200),
    usergroup char(50) NOT NULL,
    primary key (usergroup) )
;
CREATE TABLE zsd_core_c2_grpf (
    func char(50) NOT NULL,
    groupname char(50) NOT NULL,
    primary key (func,groupname) )
;
CREATE TABLE zsd_core_c2_grpu (
    groupname char(50) NOT NULL,
    uname char(12) NOT NULL,
    primary key (groupname,uname) )
;
CREATE TABLE zsd_core_catsw (
    auart string NOT NULL,
    ekgrp char(3) NOT NULL,
    item_categ_beu char(4) NOT NULL,
    item_categ_noc string NOT NULL,
    vkorg string NOT NULL,
    primary key (auart,ekgrp,item_categ_noc,vkorg) )
;
CREATE TABLE zsd_core_change (
    add_attr char(10),
    add_data char(100),
    banfn char(10),
    bnfpo numc(5),
    change string,
    ebeln char(10),
    ekgrp char(3),
    ekorg char(4),
    etenr numc(4),
    po_status char(1),
    posnr numc(6))
;
CREATE TABLE zsd_core_chk_s (
    objnr char(18),
    objtp char(5),
    remark char(50),
    status char(1))
;
CREATE TABLE zsd_core_command (
    activated char(1),
    class char(30),
    param1 char(50),
    param2 char(50),
    ucomm char(70) NOT NULL,
    primary key (ucomm) )
;
CREATE TABLE zsd_core_conf (
    param char(50) NOT NULL,
    value char(300),
    primary key (param) )
;
CREATE TABLE zsd_core_context (
    objky char(70),
    objtp char(10))
;
CREATE TABLE zsd_core_crd_blk_s (
    cmgst string,
    created_by char(12),
    created_on dats(8),
    id char(20),
    knkli char(10),
    land char(3),
    message char(220),
    name1 char(30),
    number numc(3),
    ort01 char(25),
    selected char(1),
    skfor curr(15),
    spart char(2),
    ssobl curr(15),
    sts_icn char(100),
    type char(1),
    vbeln char(10),
    vbeln_v char(10),
    vkorg char(4),
    vtweg char(2))
;
CREATE TABLE zsd_core_cust (
    auart char(4) NOT NULL,
    augru char(3) NOT NULL,
    bukrs char(4) NOT NULL,
    bzirk char(6) NOT NULL,
    ekgrp char(3) NOT NULL,
    ekorg string NOT NULL,
    kunnr char(10) NOT NULL,
    kunwe char(10) NOT NULL,
    kunwe_copy_special string NOT NULL,
    lifnr char(10) NOT NULL,
    nocorg char(4) NOT NULL,
    spart char(2) NOT NULL,
    spart_beu char(2) NOT NULL,
    vkbur char(4) NOT NULL,
    vkorg_beu char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    vtweg_beu char(2) NOT NULL,
    werk char(4) NOT NULL,
    primary key (auart,augru,bukrs,bzirk,ekgrp,ekorg,nocorg,spart,vkbur,vtweg,werk) )
;
CREATE TABLE zsd_core_cust_sw_wa (
    deal char(1),
    description char(1000),
    individual char(1),
    machine char(1),
    purchasing_group char(3),
    purchasing_group_t char(18),
    release_method char(1),
    sales_org char(4),
    scope char(1))
;
CREATE TABLE zsd_core_cx2 (
    active char(1) NOT NULL,
    auart char(4) NOT NULL,
    bsark char(4) NOT NULL,
    vkorg char(4) NOT NULL,
    vsbed char(2) NOT NULL,
    primary key (auart,bsark,vkorg,vsbed) )
;
CREATE TABLE zsd_core_dialog_attr (
    auto_expand string,
    errors_only table_of_zbc_proc_code,
    flag_core string,
    flag_ret0 string,
    flag_sync string,
    ignore_complete string,
    inv_receipt_failed string,
    level string,
    max_orders int4(10),
    show_date string,
    show_proc_code string,
    show_tech_info string)
;
CREATE TABLE zsd_core_dialog_attr_proc_code (
    flag_core string,
    flag_ret0 string,
    flag_sync string)
;
CREATE TABLE zsd_core_ebeln_data (
    ebeln char(10),
    ebelp numc(5),
    meins unit(3),
    menge quan(13))
;
CREATE TABLE zsd_core_ekko (
    ebeln char(10))
;
CREATE TABLE zsd_core_ekpo (
    ebeln char(10),
    ebelp numc(5))
;
CREATE TABLE zsd_core_eml (
    active string,
    eml_group char(50) NOT NULL,
    recipient char(241) NOT NULL,
    primary key (eml_group,recipient) )
;
CREATE TABLE zsd_core_equi_po_assignment (
    ebeln char(10),
    ebelp numc(5),
    equnr char(18))
;
CREATE TABLE zsd_core_err_ign (
    active char(1),
    msgid char(20) NOT NULL,
    msgno char(3) NOT NULL,
    proc_code char(5) NOT NULL,
    primary key (msgid,msgno,proc_code) )
;
CREATE TABLE zsd_core_fi_invoices (
    global_proc_id char(32),
    vbeln_beu_va char(10),
    vbeln_beu_vf char(10),
    vbeln_beu_vl char(10),
    vbeln_noc_va char(10),
    vkorg char(4),
    vtweg char(2))
;
CREATE TABLE zsd_core_fld_grp (
    active char(1),
    field_group char(50) NOT NULL,
    field_pos int2(5) NOT NULL,
    name char(30),
    width int2(5),
    primary key (field_group,field_pos) )
;
CREATE TABLE zsd_core_flow_s (
    abges fltp(16),
    abgru char(2),
    aedat dats(8),
    bdart char(2),
    brgew quan(15),
    bwart char(3),
    cmeth string,
    erdat dats(8),
    erzet tims(6),
    fktyp string,
    fplnr char(10),
    fpltr numc(6),
    gewei unit(3),
    hlevel int4(10),
    kzbef char(1),
    lgnum char(3),
    logsys char(10),
    matnr char(40),
    meins unit(3),
    mjahr numc(4),
    ntgew quan(13),
    plart char(1),
    plmin string,
    posnn numc(6),
    posnv numc(6),
    rfmng quan(15),
    rfmng_flo fltp(16),
    rfmng_flt fltp(16),
    rfwrt curr(15),
    sobkz char(1),
    sonum char(16),
    stufe numc(2),
    taqui char(1),
    vbeln char(10),
    vbelv char(10),
    vbtyp_n string,
    vbtyp_v char(5),
    voleh unit(3),
    volum quan(15),
    vrkme unit(3),
    waers cuky(5),
    wbsta char(1))
;
CREATE TABLE zsd_core_header_dialogs (
    add_data char(1300),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    auart char(4),
    ekgrp char(3),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    expand char(1),
    gbstk char(1),
    gjahr numc(4),
    global_proc_id char(32),
    instance_name char(30),
    key1 char(40),
    key2 char(40),
    key3 char(40),
    last_step_count int4(10),
    light char(4),
    locked string,
    mark char(1),
    no_start_before dec(15),
    noc_code string,
    obj_tab char(30),
    proc_code char(5),
    proc_counter char(32),
    proc_type char(1),
    spart char(2),
    vicon char(4),
    vkorg char(4),
    vtweg char(2))
;
CREATE TABLE zsd_core_item_assignment (
    abgru_beu char(2),
    banfn_noc char(10),
    bnfpo_noc numc(5),
    eban_menge_noc quan(13),
    ebeln_beu char(10),
    ebeln_noc char(10),
    ebelp_beu numc(5),
    ebelp_noc numc(5),
    edatu_beu dats(8),
    ekgrp char(3),
    ekpo_menge_beu quan(13),
    ekpo_menge_noc quan(13),
    etenr_noc numc(4),
    goods_movement_beu char(1),
    kosta_beu char(1),
    kosta_bez_beu char(20),
    lfimg_beu quan(13),
    lifnr_beu char(10),
    lifnr_name_beu char(35),
    lifsp_beu char(2),
    lips_menge_beu quan(13),
    matnr_beu char(18),
    meins unit(3),
    missing_meng_beu quan(13),
    posnr_beu numc(6),
    posnr_noc numc(6),
    posnr_vf_beu numc(6),
    posnr_vl_beu numc(6),
    vbap_menge_beu quan(13),
    vbeln_beu char(10),
    vbeln_noc char(10),
    vbeln_vf_beu char(10),
    vbeln_vl_beu char(10),
    vbep_menge_noc quan(13),
    vbrp_menge_beu quan(13),
    wadat_beu dats(8),
    wbsta_beu char(1),
    wbsta_bez_beu char(20),
    werks_beu char(4))
;
CREATE TABLE zsd_core_item_s (
    matnr char(40),
    meins unit(3),
    menge quan(15),
    netpr curr(11),
    noc_po_item numc(5),
    posnr numc(6),
    vbeln char(10),
    waerk cuky(5))
;
CREATE TABLE zsd_core_key (
    doc_counter int4(10),
    proc_header zbc_proc_header,
    step char(4))
;
CREATE TABLE zsd_core_lips (
    lfimg quan(13),
    matnr char(18),
    posnr_vl numc(6),
    vbeln_vl char(10))
;
CREATE TABLE zsd_core_mig_chk_s (
    auart char(4),
    cmgst string,
    dpc_icn char(100),
    proc_counter char(32),
    selected char(1),
    sts_icn char(100),
    vbeln char(10),
    vkorg char(4))
;
CREATE TABLE zsd_core_noc_order (
    arktx char(40),
    auart char(4),
    ebeln char(10),
    ekgrp char(3),
    posnr numc(6),
    spart char(2),
    uepos numc(6),
    vbeln char(10),
    vkorg char(4),
    vtweg char(2))
;
CREATE TABLE zsd_core_node (
    class char(30),
    fieldname char(30) NOT NULL,
    pos1 numc(2),
    pos2 numc(2),
    pos3 numc(2),
    pos4 numc(2),
    primary key (fieldname) )
;
CREATE TABLE zsd_core_node_ex (
    class char(30),
    cond char(60),
    fieldname char(30) NOT NULL,
    num numc(2) NOT NULL,
    primary key (fieldname,num) )
;
CREATE TABLE zsd_core_nodetxt (
    field char(30) NOT NULL,
    primary key (field) )
;
CREATE TABLE zsd_core_options (
    opt char(100))
;
CREATE TABLE zsd_core_ordctl (
    update_blocked char(1),
    vbeln char(10) NOT NULL,
    primary key (vbeln) )
;
CREATE TABLE zsd_core_pr_rel (
    auart char(4) NOT NULL,
    ekgrp char(3) NOT NULL,
    pr_unblck string,
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (auart,ekgrp,spart,vkorg) )
;
CREATE TABLE zsd_core_pr_relv (
    auart char(4) NOT NULL,
    ekgrp char(3) NOT NULL,
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (auart,ekgrp,spart,vkorg) )
;
CREATE TABLE zsd_core_presel (
    auart char(4) NOT NULL,
    ekgrp char(3) NOT NULL,
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (auart,ekgrp,spart,vkorg) )
;
CREATE TABLE zsd_core_pstyv (
    auart char(4) NOT NULL,
    direct_po char(1),
    pstyv char(4) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (auart,pstyv,vkorg) )
;
CREATE TABLE zsd_core_ranges (
    field_name char(30),
    high char(45),
    low char(45),
    option char(2),
    sign char(1))
;
CREATE TABLE zsd_core_ref_inv_data (
    aland char(3),
    bmeng quan(13),
    brtwr curr(15),
    compl string,
    ebeln char(10),
    ebelp numc(5),
    fkdat dats(8),
    meins unit(3),
    menge_inv quan(13),
    mwsk1 char(2),
    netwr curr(13),
    ref_in char(10))
;
CREATE TABLE zsd_core_rsys (
    dest char(32),
    name char(30) NOT NULL,
    org char(4) NOT NULL,
    primary key (name,org) )
;
CREATE TABLE zsd_core_select_opt (
    sign string,
    value char(4))
;
CREATE TABLE zsd_core_serail (
    enable_equi string,
    serail char(4) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (serail,vkorg) )
;
CREATE TABLE zsd_core_spras (
    bezei char(50),
    field string NOT NULL,
    spras lang(1) NOT NULL,
    primary key (field,spras) )
;
CREATE TABLE zsd_core_step_s (
    bezei char(50),
    proc_code char(5),
    step char(4))
;
CREATE TABLE zsd_core_swrel_disp1 (
    deal char(10),
    docs strg(0),
    gmt_check strg(0),
    message strg(0),
    msgid char(20),
    msgno char(3),
    order_date dats(8),
    processing_date dats(8),
    processing_time tims(6),
    req_del_date char(10),
    salesorder char(10),
    status strg(0),
    step char(4),
    triggering_order char(10))
;
CREATE TABLE zsd_core_swrel_status (
    actualdeliveryquantity quan(13),
    deliverydocument char(10),
    deliverydocumentitem numc(6),
    deliveryquantityunit unit(3),
    deliverystatus char(1),
    goodsmovementreasoncode numc(4),
    goodsmovementstatus char(1),
    goodsmovementtype char(3),
    material char(18),
    pickingconfirmationstatus char(1),
    pickingcontrol string,
    pickingstatus char(1),
    purchaseconfirmationstatus string,
    salesorder char(10),
    salesorderitem numc(6),
    sddocumentcollectivenumber char(10),
    shippingtype char(2),
    z_noc_salesorder char(10))
;
CREATE TABLE zsd_core_tax (
    bukrs char(4) NOT NULL,
    gl_account char(10),
    tax_code char(2),
    primary key (bukrs) )
;
CREATE TABLE zsd_core_tree_data_s (
    bezei char(50),
    doc_counter int4(10),
    event char(32),
    light char(4),
    mark string,
    msgli char(220),
    node_key strg(0),
    objky char(70),
    objtp char(10),
    vicon char(4))
;
CREATE TABLE zsd_core_tst_ord (
    created_by char(12),
    created_on dats(8),
    sys char(32) NOT NULL,
    vbeln char(10) NOT NULL,
    vbeln_v char(10),
    primary key (sys,vbeln) )
;
CREATE TABLE zsd_core_tst_ord_s (
    created_by char(12),
    created_on dats(8),
    id char(20),
    message char(220),
    number numc(3),
    selected char(1),
    sys char(32),
    type char(1),
    vbeln char(10),
    vbeln_v char(10))
;
CREATE TABLE zsd_core_vbak (
    bstnk char(20),
    spart char(2),
    vbeln char(10),
    vkbur char(4),
    vkgrp char(3),
    vkorg char(4),
    vtweg char(2))
;
CREATE TABLE zsd_core_vbap (
    posnr numc(6),
    vbeln char(10))
;
CREATE TABLE zsd_core_vbap_s (
    kwmeng quan(15),
    matnr char(18),
    posnr numc(6),
    vbeln char(10),
    vrkme unit(3))
;
CREATE TABLE zsd_core_ver_map (
    auart char(4) NOT NULL,
    matnr char(18) NOT NULL,
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    vsbed char(2) NOT NULL,
    vsbed_ecc char(2) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,matnr,spart,vkorg,vsbed_ecc,vtweg) )
;
CREATE TABLE zsd_core_ver_mav (
    auart char(4) NOT NULL,
    matnr char(18) NOT NULL,
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    vsbed_ecc char(2) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,matnr,spart,vkorg,vsbed_ecc,vtweg) )
;
CREATE TABLE zsd_core_vko (
    vkorg string NOT NULL,
    primary key (vkorg) )
;
CREATE TABLE zsd_core_vko_v (
    vkorg string NOT NULL,
    vtext char(20),
    primary key (vkorg) )
;
CREATE TABLE zsd_credit_check (
    auart char(4) NOT NULL,
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,spart,vkorg,vtweg) )
;
CREATE TABLE zsd_cret01_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    ref_data char(30),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_cret01_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_cret01_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_cret02_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    ref_data char(30),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_cret02_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_cret02_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_csync1_docs (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_csync1_mess (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_csync1_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    banfn char(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_csync2_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    ebeln char(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,ebeln,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_csync2_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    ebeln char(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,ebeln,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_csync2_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    ebeln char(10) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (ebeln,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_cust (
    zz_katr6 char(3),
    zz_katr8 char(3),
    zz_kdkg1 char(2))
;
CREATE TABLE zsd_cust_mail (
    email char(241) NOT NULL,
    kschl char(4) NOT NULL,
    kunnr char(10) NOT NULL,
    lfdnr char(3) NOT NULL,
    spart string NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    primary key (kschl,lfdnr,spart,vkorg,vtweg) )
;
CREATE TABLE zsd_cust_resp (
    responsible char(3) NOT NULL,
    primary key (responsible) )
;
CREATE TABLE zsd_cust_resp_t (
    description char(30),
    responsible string NOT NULL,
    spras string NOT NULL,
    primary key (responsible,spras) )
;
CREATE TABLE zsd_custlist (
    abtnr char(4),
    adrnr char(10),
    bzirk char(6),
    distr char(40),
    kdgrp char(2),
    ktokd char(4),
    kunnr char(10),
    kvgr2 char(3),
    kvgr5 char(3),
    land1 char(3),
    loc char(40),
    loevm char(1),
    loevmkna1 char(1),
    lzone char(10),
    main_cust char(1),
    name1 char(35),
    name2 char(35),
    name3 char(35),
    name4 char(35),
    name_co char(40),
    ort01 char(35),
    parnr numc(10),
    prat1 string,
    prat2 char(1),
    prat3 char(1),
    prat4 char(1),
    prat5 char(1),
    prat6 char(1),
    prat7 char(1),
    prat8 char(1),
    prat9 char(1),
    prata char(1),
    pstlz char(10),
    smtp_addr char(241),
    sort2 char(20),
    spart char(2),
    stcd1 char(16),
    stceg char(20),
    stras char(35),
    supp1 char(40),
    supp2 char(40),
    supp3 char(40),
    vkbur char(4),
    vkgrp char(3),
    vkorg char(4),
    vtext char(30),
    vtweg char(2),
    zterm char(4),
    zzkunnr_par char(10))
;
CREATE TABLE zsd_data_transf (
    zz_data_trf char(1))
;
CREATE TABLE zsd_data_transfer (
    zz_data_trf char(1))
;
CREATE TABLE zsd_deliveries_alv (
    brgew quan(15),
    esa char(8),
    gewei unit(3),
    kurs dec(9),
    lfimg quan(13),
    maktx char(40),
    matnr char(18),
    peinh dec(5),
    posnr numc(6),
    stawn char(17),
    stprs curr(11),
    sumgw quan(15),
    ttprh curr(11),
    ttprs curr(11),
    vbeln char(10),
    vrkme unit(3))
;
CREATE TABLE zsd_deliveries_for_order (
    anzahl_lieferungen numc(3),
    erdat dats(8),
    posnr numc(6),
    vbeln char(10),
    vbelv char(10),
    vkorg char(4),
    werks char(4))
;
CREATE TABLE zsd_dispatch_note_list (
    arktx char(40),
    bolnr char(35),
    inco1 string,
    inco2 char(28),
    kosch char(18),
    kunag string,
    kunnr string,
    labst quan(13),
    lddat dats(8),
    lf_mg quan(13),
    lfdat dats(8),
    lfimg quan(13),
    lgort string,
    lifnr string,
    light string,
    matnr char(18),
    meins unit(3),
    menga quan(13),
    mengb quan(13),
    mengc quan(13),
    mengd quan(13),
    menge quan(13),
    of_au dec(5),
    of_mg quan(13),
    pprop char(1),
    stcar numc(5),
    traty string,
    vbeln char(10) NOT NULL,
    verur char(35),
    vgbel char(10),
    vgpos numc(6),
    vkorg string,
    vstel string,
    wadat dats(8),
    werks string,
    zz_carrier char(5),
    zz_eta dats(8),
    zz_etd dats(8),
    zz_f_harbour char(20),
    zz_osanr char(35),
    zz_t_harbour char(20),
    zz_v_harbour char(20),
    zz_vd_eta dats(8),
    zz_vessel char(30),
    primary key (vbeln) )
;
CREATE TABLE zsd_doc_textedit_v_print (
    id char(4),
    v1 string,
    v2 string,
    v3 string)
;
CREATE TABLE zsd_e2e_data (
    abgru_beu char(2),
    abgru_noc char(2),
    auart_noc char(4),
    banfn_noc char(10),
    bbein_noc dats(8),
    bnfpo_noc numc(5),
    bstnk_noc char(20),
    eban_menge_noc quan(13),
    ebeln_beu char(10),
    ebeln_noc char(10),
    ebelp_beu numc(5),
    ebelp_noc numc(5),
    edatu_beu dats(8),
    edatu_noc dats(8),
    ekes_menge_noc quan(13),
    ekgrp_noc char(3),
    ekpo_menge_beu quan(13),
    ekpo_menge_noc quan(13),
    erdat_beu dats(8),
    erdat_noc dats(8),
    erdat_vf_beu dats(8),
    erdat_vl_beu dats(8),
    ernam_beu char(12),
    ernam_noc char(12),
    erzet_beu tims(6),
    erzet_noc tims(6),
    erzet_vl_beu tims(6),
    faksp_noc char(2),
    fksta_bez_noc char(20),
    fksta_noc char(1),
    gbsta_bez_noc char(20),
    gbsta_noc char(1),
    goods_movement_beu char(1),
    kosta_beu char(1),
    kosta_bez_beu char(20),
    kunag_beu char(10),
    kunag_name_beu char(35),
    kunag_name_noc char(35),
    kunag_noc char(10),
    kunve_name_noc char(35),
    kunve_noc numc(8),
    kunwe_beu char(10),
    kunwe_name_beu char(35),
    kunwe_name_noc char(35),
    kunwe_noc char(10),
    lfimg_beu quan(13),
    lifnr_beu char(10),
    lifnr_name_beu char(35),
    lifsp_beu char(2),
    lifsp_noc char(2),
    lips_menge_beu quan(13),
    maktx_noc char(40),
    matkl_noc char(9),
    matnr_noc char(40),
    meins unit(3),
    missing_meng_beu quan(13),
    missing_netpr_noc curr(15),
    mtart_noc char(4),
    netwr_noc curr(15),
    posnr_beu numc(6),
    posnr_noc numc(6),
    project_order char(24),
    qmnum_noc char(12),
    re_belnr_noc char(10),
    spart_beu char(2),
    spart_noc char(2),
    vbap_menge_beu quan(13),
    vbeln_beu char(10),
    vbeln_noc char(10),
    vbeln_vf_beu char(10),
    vbeln_vf_noc char(10),
    vbeln_vl_beu char(10),
    vbeln_vl_noc char(10),
    vbep_menge_noc quan(13),
    vbrp_menge_beu quan(13),
    vdatu_noc dats(8),
    vkbur_beu char(4),
    vkbur_noc char(4),
    vkgrp_beu char(3),
    vkgrp_noc char(3),
    vkorg_beu char(4),
    vkorg_noc char(4),
    vsbed_noc char(2),
    vsbed_vl_beu char(2),
    vtweg_beu char(2),
    vtweg_noc char(2),
    wadat_ist_vl_beu dats(8),
    waerk_noc cuky(5),
    wbsta_beu char(1),
    wbsta_bez_beu char(20),
    wbsta_bez_noc char(20),
    wbsta_noc char(1),
    we_belnr_noc char(10),
    werks_beu char(4))
;
CREATE TABLE zsd_e2e_ekbe (
    belnr char(10),
    ebeln char(10),
    ebelp numc(5),
    gjahr numc(4),
    vgabe string)
;
CREATE TABLE zsd_e2e_item_assignment (
    banfn_noc char(10),
    bnfpo_noc numc(5),
    eban_menge_noc quan(13),
    ebeln_beu char(10),
    ebeln_noc char(10),
    ebelp_beu numc(5),
    ebelp_noc numc(5),
    edatu_beu dats(8),
    ekgrp char(3),
    ekpo_menge_beu quan(13),
    ekpo_menge_noc quan(13),
    erdat_beu dats(8),
    erdat_vf_beu dats(8),
    erdat_vl_beu dats(8),
    ernam_beu char(12),
    erzet_beu tims(6),
    erzet_vl_beu tims(6),
    etenr_noc numc(4),
    goods_movement_beu char(1),
    kunag_beu char(10),
    kunag_name_beu char(30),
    kunwe_beu char(10),
    kunwe_name_beu char(30),
    lips_menge_beu quan(13),
    meins unit(3),
    posnr_beu numc(6),
    posnr_noc numc(6),
    posnr_vf_beu numc(6),
    posnr_vl_beu numc(6),
    spart_beu char(2),
    vbap_menge_beu quan(13),
    vbeln_beu char(10),
    vbeln_noc char(10),
    vbeln_vf_beu char(10),
    vbeln_vl_beu char(10),
    vbep_menge_noc quan(13),
    vbrp_menge_beu quan(13),
    vkbur_beu char(4),
    vkgrp_beu char(3),
    vkorg_beu char(4),
    vsbed_vl_beu char(2),
    vtweg_beu char(2),
    wadat_ist_vl_beu dats(8))
;
CREATE TABLE zsd_ebiz2_ordf02 (
    kunnr string NOT NULL,
    lifsk char(2),
    matkl char(9) NOT NULL,
    tdid string,
    textname char(70),
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    primary key (kunnr,matkl,vkorg,vtweg) )
;
CREATE TABLE zsd_ebiz_block (
    auart char(4) NOT NULL,
    check_field char(30) NOT NULL,
    check_table char(16) NOT NULL,
    faksk char(2),
    lifsk char(2),
    spart char(2) NOT NULL,
    value char(30),
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,check_field,check_table,spart,vkorg,vtweg) )
;
CREATE TABLE zsd_engineer_calendar (
    kunnr char(10) NOT NULL,
    lgort char(4) NOT NULL,
    mark string,
    month_from string,
    month_name_01 char(3),
    month_name_02 char(3),
    month_name_03 char(3),
    month_to string,
    stat string NOT NULL,
    t01 char(1),
    t02 char(1),
    t03 char(1),
    t04 char(1),
    t05 char(1),
    t06 char(1),
    t07 char(1),
    t08 char(1),
    t09 char(1),
    t10 char(1),
    t11 char(1),
    t12 char(1),
    t13 char(1),
    t14 char(1),
    t15 char(1),
    t16 char(1),
    t17 char(1),
    t18 char(1),
    t19 char(1),
    t20 char(1),
    t21 char(1),
    t22 char(1),
    t23 char(1),
    t24 char(1),
    t25 char(1),
    t26 char(1),
    t27 char(1),
    t28 char(1),
    t29 char(1),
    t30 char(1),
    t31 char(1),
    t32 char(1),
    t33 char(1),
    t34 char(1),
    t35 char(1),
    t36 char(1),
    t37 char(1),
    t38 char(1),
    t39 char(1),
    t40 char(1),
    t41 char(1),
    t42 char(1),
    t43 char(1),
    t44 char(1),
    t45 char(1),
    t46 char(1),
    t47 char(1),
    t48 char(1),
    t49 char(1),
    t50 char(1),
    t51 char(1),
    t52 char(1),
    t53 char(1),
    t54 char(1),
    t55 char(1),
    t56 char(1),
    t57 char(1),
    t58 char(1),
    t59 char(1),
    t60 char(1),
    t61 char(1),
    t62 char(1),
    t63 char(1),
    t64 char(1),
    t65 char(1),
    t66 char(1),
    t67 char(1),
    t68 char(1),
    t69 char(1),
    t70 char(1),
    t71 char(1),
    t72 char(1),
    t73 char(1),
    t74 char(1),
    t75 char(1),
    t76 char(1),
    t77 char(1),
    t78 char(1),
    t79 char(1),
    t80 char(1),
    t81 char(1),
    t82 char(1),
    t83 char(1),
    t84 char(1),
    t85 char(1),
    t86 char(1),
    t87 char(1),
    t88 char(1),
    t89 char(1),
    t90 char(1),
    t91 char(1),
    t92 char(1),
    t93 char(1),
    updkz char(1),
    wefnr char(10) NOT NULL,
    werks char(4) NOT NULL,
    year_from numc(4),
    year_to numc(4),
    primary key (kunnr,lgort,stat,wefnr,werks) )
;
CREATE TABLE zsd_engineer_calendar_days (
    t01 char(1),
    t02 char(1),
    t03 char(1),
    t04 char(1),
    t05 char(1),
    t06 char(1),
    t07 char(1),
    t08 char(1),
    t09 char(1),
    t10 char(1),
    t11 char(1),
    t12 char(1),
    t13 char(1),
    t14 char(1),
    t15 char(1),
    t16 char(1),
    t17 char(1),
    t18 char(1),
    t19 char(1),
    t20 char(1),
    t21 char(1),
    t22 char(1),
    t23 char(1),
    t24 char(1),
    t25 char(1),
    t26 char(1),
    t27 char(1),
    t28 char(1),
    t29 char(1),
    t30 char(1),
    t31 char(1),
    t32 char(1),
    t33 char(1),
    t34 char(1),
    t35 char(1),
    t36 char(1),
    t37 char(1),
    t38 char(1),
    t39 char(1),
    t40 char(1),
    t41 char(1),
    t42 char(1),
    t43 char(1),
    t44 char(1),
    t45 char(1),
    t46 char(1),
    t47 char(1),
    t48 char(1),
    t49 char(1),
    t50 char(1),
    t51 char(1),
    t52 char(1),
    t53 char(1),
    t54 char(1),
    t55 char(1),
    t56 char(1),
    t57 char(1),
    t58 char(1),
    t59 char(1),
    t60 char(1),
    t61 char(1),
    t62 char(1),
    t63 char(1),
    t64 char(1),
    t65 char(1),
    t66 char(1),
    t67 char(1),
    t68 char(1),
    t69 char(1),
    t70 char(1),
    t71 char(1),
    t72 char(1),
    t73 char(1),
    t74 char(1),
    t75 char(1),
    t76 char(1),
    t77 char(1),
    t78 char(1),
    t79 char(1),
    t80 char(1),
    t81 char(1),
    t82 char(1),
    t83 char(1),
    t84 char(1),
    t85 char(1),
    t86 char(1),
    t87 char(1),
    t88 char(1),
    t89 char(1),
    t90 char(1),
    t91 char(1),
    t92 char(1),
    t93 char(1))
;
CREATE TABLE zsd_engineer_cust (
    btag string,
    die string,
    don char(1),
    fre char(1),
    ident char(2),
    kunnr char(10) NOT NULL,
    lgort char(4) NOT NULL,
    mark string,
    mit char(1),
    mon char(1),
    name char(30),
    persnr char(8),
    sam char(1),
    son char(1),
    stat string NOT NULL,
    updkz char(1),
    wefnr char(10) NOT NULL,
    werks char(4) NOT NULL,
    primary key (kunnr,lgort,stat,wefnr,werks) )
;
CREATE TABLE zsd_engineer_key (
    kunnr char(10) NOT NULL,
    lgort char(4) NOT NULL,
    stat string NOT NULL,
    wefnr char(10) NOT NULL,
    werks char(4) NOT NULL,
    primary key (kunnr,lgort,stat,wefnr,werks) )
;
CREATE TABLE zsd_equis_action (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    endda dats(8),
    event char(32),
    proc_type string NOT NULL,
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,proc_type,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_equis_cls (
    characteristic char(30),
    cond char(60),
    id numc(6) NOT NULL,
    matnr char(18),
    value char(30),
    primary key (id) )
;
CREATE TABLE zsd_equis_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    vbeln_vl char(10) NOT NULL,
    primary key (doc_counter,gjahr,proc_code,proc_counter,step,vbeln,vbeln_vl) )
;
CREATE TABLE zsd_equis_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    vbeln_vl char(10) NOT NULL,
    primary key (doc_counter,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln,vbeln_vl) )
;
CREATE TABLE zsd_equis_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_equis_st_t (
    bezei char(50) NOT NULL,
    spras string NOT NULL,
    step string NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_equis_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    vbeln_vl char(10) NOT NULL,
    primary key (gjahr,proc_code,proc_counter,step,vbeln,vbeln_vl) )
;
CREATE TABLE zsd_eu_exit (
    datab dats(8),
    land1 char(3) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (land1,vkorg) )
;
CREATE TABLE zsd_euvat (
    parvw char(2) NOT NULL,
    vkorg string NOT NULL,
    primary key (vkorg) )
;
CREATE TABLE zsd_evers (
    ekorg string NOT NULL,
    evers string,
    expvz string NOT NULL,
    kunnr string NOT NULL,
    route string NOT NULL,
    primary key (ekorg,expvz,kunnr,route) )
;
CREATE TABLE zsd_exitmethod_checkfields (
    fieldname char(30),
    fieldvalue char(40))
;
CREATE TABLE zsd_exitmethod_params (
    cs_svbap int4,
    kuwev char(10),
    rv45a unknown,
    vbak vbak,
    vbap vbap,
    xvbap_t table_of_vbap)
;
CREATE TABLE zsd_exitmethods (
    classname string NOT NULL,
    method_id numc(4) NOT NULL,
    methodname string NOT NULL,
    primary key (classname,method_id,methodname) )
;
CREATE TABLE zsd_exitprocess (
    process_id char(4) NOT NULL,
    primary key (process_id) )
;
CREATE TABLE zsd_exitprocesst (
    process_id string NOT NULL,
    spras lang(1) NOT NULL,
    text char(40),
    primary key (process_id,spras) )
;
CREATE TABLE zsd_exitsequence (
    fieldname char(30),
    fieldvalue char(40),
    method_id string NOT NULL,
    seq_order numc(2) NOT NULL,
    sequence_id char(4) NOT NULL,
    primary key (seq_order,sequence_id) )
;
CREATE TABLE zsd_ext_lips_ic (
    posnr numc(6),
    zz_beu_del char(10),
    zz_beu_del_date dats(8),
    zz_beu_pos numc(6))
;
CREATE TABLE zsd_faktcheck (
    ernam char(12),
    fkdat dats(8),
    permi char(16),
    vbeln char(10),
    vbelnr char(10),
    vstat string)
;
CREATE TABLE zsd_faktlist (
    arktx char(40),
    aubel char(10),
    bzirk char(6),
    fkdat dats(8),
    knumv char(10),
    kunag char(10),
    kunwe char(10),
    matnr char(18),
    nameag char(30),
    namewe char(30),
    netwr curr(15),
    posnr numc(6),
    vbeln char(10),
    vkbur char(4),
    vkgrp char(3),
    vkorg char(4),
    vtweg char(2),
    wavwr curr(13))
;
CREATE TABLE zsd_faktlist_nach (
    aubel char(10),
    bzirk char(6),
    datvr dats(8),
    fkdat dats(8),
    kschl char(4),
    kunag char(10),
    nameag char(30),
    tdline char(132),
    uhrvr tims(6),
    vbeln char(10),
    vkbur char(4),
    vkgrp char(3),
    vkorg char(4),
    vstat string,
    vsztp string,
    vtweg char(2))
;
CREATE TABLE zsd_fields_block (
    active string,
    directory char(100),
    fieldname char(50) NOT NULL,
    primary key (fieldname) )
;
CREATE TABLE zsd_first_store (
    etenr numc(4) NOT NULL,
    posnr numc(6) NOT NULL,
    vbeln char(10) NOT NULL,
    z1datum dats(8),
    z1meng quan(13),
    z1zeit tims(6),
    primary key (etenr,posnr,vbeln) )
;
CREATE TABLE zsd_forinv_display (
    aland char(3),
    arktx char(40),
    autyp string,
    bukrs char(4),
    expkz char(1),
    fkart char(4),
    fkdat dats(8),
    fkimg quan(13),
    fktyp string,
    kbetr curr(11),
    kkber char(4),
    knumv char(10),
    konwa cuky(5),
    kschl char(4),
    ktokd_re char(4),
    ktokd_rg char(4),
    ktokd_we char(4),
    kunag char(10),
    kunre char(10),
    kunrg char(10),
    kunwe char(10),
    land1 char(3),
    land_eq char(3),
    land_re char(3),
    land_rg char(3),
    land_we char(3),
    landtx char(3),
    logsys char(10),
    matkl char(9),
    matnr char(18),
    mwsbk curr(13),
    mwsk1 char(2),
    mwskz char(2),
    name_re char(30),
    name_rg char(30),
    name_we char(30),
    netwr curr(15),
    netwr_h curr(15),
    plz_re char(10),
    plz_rg char(10),
    plz_we char(10),
    posnr numc(6),
    posnv numc(6),
    pstyv char(4),
    spart char(2),
    stcd1_re char(16),
    stcd1_rg char(16),
    stcd1_we char(16),
    stcd2_re char(11),
    stcd2_rg char(11),
    stcd2_we char(11),
    stceg char(20),
    stceg_h string,
    stceg_l char(3),
    stceg_re char(20),
    stceg_rg char(20),
    stceg_we char(20),
    stkza_re string,
    stkza_rg string,
    stkza_we string,
    stkzu_re char(1),
    stkzu_rg char(1),
    stkzu_we char(1),
    taxk1 char(1),
    taxk2 char(1),
    taxm1 char(1),
    taxm2 char(1),
    taxmt char(1),
    vbeln char(10) NOT NULL,
    vbelv char(10),
    vbtyp string,
    vgtyp char(3),
    vkorg char(4),
    vrkme unit(3),
    vstel char(4),
    vtweg char(2),
    waerk cuky(5),
    werks char(4),
    xegdr char(1),
    xegld_eq char(1),
    xegld_rg char(1),
    xegld_we char(1),
    primary key (vbeln) )
;
CREATE TABLE zsd_gelangen (
    email char(241),
    kunnr char(10) NOT NULL,
    primary key (kunnr) )
;
CREATE TABLE zsd_goods_purch_sel_price_rep (
    ebeln char(10),
    ebelp numc(5),
    ekorg char(4),
    lifnr char(10),
    maktx char(40),
    matnr char(18),
    name1 char(30),
    netwr_fp curr(15),
    posnr_vf numc(6),
    stprs curr(11),
    vbeln char(10),
    vbeln_vf char(10),
    vbelp numc(6),
    zsd_pfpic curr(11),
    zsd_pppo curr(11),
    zsd_soitm_netpr curr(11),
    zsd_zepm curr(11),
    zsd_zpd5 curr(11),
    zsd_zplm curr(11))
;
CREATE TABLE zsd_hana_ecc (
    exidv2_hana char(20) NOT NULL,
    forward_name char(30) NOT NULL,
    forward_partner char(10) NOT NULL,
    vbeln_va_ecc char(10) NOT NULL,
    vbeln_va_hana char(10) NOT NULL,
    vbeln_vl_hana char(10) NOT NULL,
    vstat_nast_ztm char(4) NOT NULL,
    primary key (exidv2_hana,vbeln_vl_hana) )
;
CREATE TABLE zsd_ic_deal_stat (
    deal char(10) NOT NULL,
    last_update dec(15),
    status char(1),
    status_dlv char(1),
    primary key (deal) )
;
CREATE TABLE zsd_ic_vbap (
    abgru string,
    absta string,
    arktx char(40),
    besta char(1),
    charg char(10),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    fksaa char(1),
    fksta char(1),
    gbsta char(1),
    kwmeng quan(15),
    lfgsa char(1),
    lfsta char(1),
    matnr string,
    posnr string NOT NULL,
    prodh string,
    pstyv string,
    uepos numc(6),
    vbeln string NOT NULL,
    vrkme string,
    wbsta char(1),
    primary key (posnr,vbeln) )
;
CREATE TABLE zsd_incl_proj_rep_vbak_koz (
    zz_lead_source char(20),
    zz_proj_nr char(24),
    zz_proj_valid string)
;
CREATE TABLE zsd_incl_proj_rep_vbak_kozx (
    zz_lead_source char(1),
    zz_proj_nr char(1),
    zz_proj_valid char(1))
;
CREATE TABLE zsd_incl_project_reporting (
    zz_lead_source char(20),
    zz_proj_nr char(24),
    zz_proj_valid string)
;
CREATE TABLE zsd_incl_project_reporting2 (
    zz_proj_contr char(24),
    zz_proj_order char(24))
;
CREATE TABLE zsd_incl_project_reporting_koz (
    zz_lead_source char(20),
    zz_proj_nr char(24),
    zz_proj_valid string)
;
CREATE TABLE zsd_incl_project_reportingkozx (
    zz_lead_source char(1),
    zz_proj_nr char(1),
    zz_proj_valid char(1))
;
CREATE TABLE zsd_incl_project_reportingx (
    zz_lead_source char(1),
    zz_proj_nr char(1),
    zz_proj_valid char(1))
;
CREATE TABLE zsd_intrastat_bit (
    arrivdepa char(100),
    arrivdepa1 char(100),
    belegnumr char(100),
    eigenmaeh char(100),
    grenzweak char(100),
    isessione char(100),
    liefruinr char(100),
    nummbukrs char(100),
    nummbukrs1 char(100),
    nummwerks char(100),
    nummwerks1 char(100),
    payer_country char(100),
    payment_code char(100),
    rechnweak char(100),
    rechnweeh char(100),
    service_code char(100),
    serviceitem char(100),
    supply_code char(100))
;
CREATE TABLE zsd_inv_mailsend (
    fkart string NOT NULL,
    kschl string NOT NULL,
    mailsender char(12),
    vkorg string NOT NULL,
    primary key (fkart,kschl,vkorg) )
;
CREATE TABLE zsd_invnr (
    fkart string NOT NULL,
    numki char(2),
    vkorg char(4) NOT NULL,
    zzil char(1) NOT NULL,
    primary key (fkart,vkorg,zzil) )
;
CREATE TABLE zsd_invoices_alv (
    arktx char(40),
    bolnr char(35),
    gewei unit(3),
    kunrg char(10),
    kurs dec(9),
    lfimg quan(13),
    matnr char(18),
    netwr curr(11),
    posnr numc(6),
    stawn char(17),
    sumgw quan(15),
    ttprnok curr(11),
    vbeln char(10),
    vgbel char(10),
    vrkme unit(3),
    waerk cuky(5),
    zfbdt dats(8))
;
CREATE TABLE zsd_invoices_excel (
    arktx char(40),
    bolnr char(35),
    gewei unit(3),
    kunrg char(10),
    kurrf dec(9),
    lfimg quan(13),
    matnr char(18),
    netwr curr(11),
    posnr numc(6),
    stawn char(17),
    sumgw quan(15),
    ttprnok curr(11),
    vbeln char(10),
    vgbel char(10),
    vrkme unit(3),
    waerk cuky(5),
    zfbdt dats(8))
;
CREATE TABLE zsd_invoices_log (
    erdat dats(8) NOT NULL,
    ernam char(12) NOT NULL,
    erzet tims(6) NOT NULL,
    logdat dats(8),
    logzet tims(6),
    vbeln char(10) NOT NULL,
    primary key (erdat,ernam,erzet,vbeln) )
;
CREATE TABLE zsd_kondkop_1 (
    kotabnr char(3) NOT NULL,
    kschl char(4) NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    zcheck string,
    zkotabnr char(3),
    zkschl char(4) NOT NULL,
    zvkorg char(4) NOT NULL,
    zvtweg char(2) NOT NULL,
    primary key (kotabnr,kschl,vkorg,vtweg,zkschl,zvkorg,zvtweg) )
;
CREATE TABLE zsd_kondkop_2 (
    aedat dats(8),
    aenam char(12),
    aezeit tims(6),
    datab dats(8) NOT NULL,
    datbi dats(8) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzeit tims(6),
    kbetr curr(11),
    kmein unit(3),
    knumh char(10) NOT NULL,
    konwa cuky(5),
    kotabnr char(3) NOT NULL,
    kpein dec(5),
    kschl string NOT NULL,
    kznep char(1),
    matnr char(18) NOT NULL,
    updkz char(1) NOT NULL,
    valdt dats(8),
    valtg numc(2),
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    zterm char(4),
    zzqcodegrp char(8) NOT NULL,
    primary key (datab,datbi,knumh,kotabnr,kschl,matnr,vkorg,vtweg,zzqcodegrp) )
;
CREATE TABLE zsd_kondkop_3 (
    aedat dats(8),
    aenam char(12),
    aezeit tims(6),
    datab dats(8) NOT NULL,
    datbi dats(8) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzeit tims(6),
    kbetr curr(11),
    klfn1 numc(4) NOT NULL,
    knumh char(10) NOT NULL,
    konpkmein unit(3),
    kopos numc(2) NOT NULL,
    kotabnr char(3) NOT NULL,
    kschl char(4) NOT NULL,
    kstbm quan(15),
    kstbw curr(15),
    matnr char(18) NOT NULL,
    rv13akonwa cuky(5),
    updkz char(1),
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    zzqcodegrp char(8) NOT NULL,
    primary key (datab,datbi,klfn1,knumh,kopos,kotabnr,kschl,matnr,vkorg,vtweg,zzqcodegrp) )
;
CREATE TABLE zsd_kpi_fa (
    aedat_rmcode_z dats(8),
    auart string,
    avi_rmca600 dec(3),
    avi_rmcz900 dec(3),
    bldat dats(8),
    diff_wld_ist dec(3),
    dlz_lif_a600_z900 dec(3),
    dlz_lif_e100_z900 dec(3),
    lfart char(4),
    lfart_tm char(1),
    nab_genlif_rmca600 dec(3),
    nab_rmca600_rmcz900 dec(3),
    rmcode_z char(3),
    spdnr char(10),
    trref_l char(10),
    udate dats(8),
    ums_genlif_rmca600 dec(3),
    ums_genlif_rmcz900 dec(3),
    ums_rmca600_rmcz900 dec(3),
    vbeln char(10),
    vdatu dats(8),
    vsbed string,
    vtweg string)
;
CREATE TABLE zsd_kunde_kopieren_0100 (
    begru char(4),
    bukrs string,
    busab string,
    busab_ma char(2),
    bzirk string,
    fdgrv string,
    hbkid string,
    hkunnr string,
    kdgrp string,
    ktokd char(4),
    kunhierlevel numc(2),
    kunnr string,
    loevm char(1),
    mahna string,
    namve char(30),
    parve string,
    spart string,
    vkbur string,
    vkgrp string,
    vkorg string,
    vtweg string)
;
CREATE TABLE zsd_kunnr_shlp (
    kunnr string,
    spart string,
    vkorg string,
    vtweg string)
;
CREATE TABLE zsd_kunntext (
    counter numc(3) NOT NULL,
    tdformat char(2),
    tdid char(4) NOT NULL,
    tdline char(132),
    tdname char(70) NOT NULL,
    tdobject char(10) NOT NULL,
    tdspras lang(1) NOT NULL,
    primary key (counter,tdid,tdname,tdobject,tdspras) )
;
CREATE TABLE zsd_liefsp_s (
    kunnr string,
    land1 string,
    maktx char(40),
    matnr char(18),
    name1 char(35),
    name2 char(35),
    posnr string,
    vbeln string,
    werks char(4),
    wmeng quan(13),
    zzlink_pin char(60),
    zzopport char(60))
;
CREATE TABLE zsd_lips_ic (
    zz_beu_del char(10),
    zz_beu_del_date dats(8),
    zz_beu_pos numc(6))
;
CREATE TABLE zsd_lpi_cert (
    datab dats(8),
    datbi dats(8) NOT NULL,
    kunag char(10) NOT NULL,
    vkorg char(4) NOT NULL,
    zcert char(20),
    zcp char(40),
    zmail char(60),
    zphone char(40),
    primary key (datbi,kunag,vkorg) )
;
CREATE TABLE zsd_mat_repl_grp (
    create_date dats(8) NOT NULL,
    create_time tims(6) NOT NULL,
    create_user char(12) NOT NULL,
    file_name char(255),
    primary key (create_date,create_time,create_user) )
;
CREATE TABLE zsd_mat_repl_itm (
    bundle char(80) NOT NULL,
    create_date dats(8) NOT NULL,
    create_time tims(6) NOT NULL,
    create_user char(12) NOT NULL,
    posnr numc(6) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (bundle,create_date,create_time,create_user,posnr,vbeln) )
;
CREATE TABLE zsd_mat_repl_msg (
    bundle char(80) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    rnum int4(10) NOT NULL,
    run_date dats(8),
    run_time tims(6),
    status char(1),
    step char(4) NOT NULL,
    primary key (bundle,rnum,step) )
;
CREATE TABLE zsd_mat_repl_out (
    exec_date dats(8),
    exec_time tims(6),
    material_new char(18),
    material_old char(18),
    message char(220),
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    posnr numc(6),
    status char(1),
    step char(4),
    vbeln char(10))
;
CREATE TABLE zsd_mat_repl_task (
    bundle char(80),
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    name char(80),
    par1 char(80),
    par2 char(80),
    status char(1),
    step char(4))
;
CREATE TABLE zsd_mat_repl_tsk (
    bundle char(80) NOT NULL,
    create_date dats(8),
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    name char(80),
    par1 char(80),
    par2 char(80),
    res1 char(80),
    run_date dats(8),
    run_time tims(6),
    status char(1),
    step char(4) NOT NULL,
    primary key (bundle,step) )
;
CREATE TABLE zsd_mat_repl_typ (
    active char(1) NOT NULL,
    auart char(4) NOT NULL,
    primary key (auart) )
;
CREATE TABLE zsd_mat_sta_eol (
    mstav char(2) NOT NULL,
    primary key (mstav) )
;
CREATE TABLE zsd_material_day (
    matkl char(9) NOT NULL,
    pgi_days int4(10),
    vkorg char(4) NOT NULL,
    primary key (matkl,vkorg) )
;
CREATE TABLE zsd_mgi_mail (
    mail char(241) NOT NULL,
    name char(40),
    primary key (mail) )
;
CREATE TABLE zsd_misc (
    zz_email_req string,
    zz_proj_no char(35))
;
CREATE TABLE zsd_ocdelta (
    fieldname string NOT NULL,
    tabname string NOT NULL,
    primary key (fieldname,tabname) )
;
CREATE TABLE zsd_ocdelta_changes (
    chngind string,
    etenr numc(4),
    fieldname char(30),
    last_change dats(8),
    last_change_time tims(6),
    posnr numc(6),
    tabname char(30),
    value_new char(254),
    value_old char(254),
    vbeln char(10))
;
CREATE TABLE zsd_odn_snum (
    blart string,
    brsch string NOT NULL,
    bukrs char(4) NOT NULL,
    fkart string NOT NULL,
    zzvart char(3) NOT NULL,
    primary key (brsch,bukrs,fkart,zzvart) )
;
CREATE TABLE zsd_oh_insert (
    aspdnr char(10),
    changer char(12),
    colli char(10),
    datum dats(8),
    del_flag char(1),
    gewei unit(3),
    kfzkz char(10),
    lgtor char(3),
    ntgew quan(15),
    ospdnr char(10),
    sammg char(10),
    state string,
    time tims(6),
    vbeln char(10),
    vtext char(30),
    werks char(4))
;
CREATE TABLE zsd_one_hub_date (
    bisdat dats(8),
    filtr char(10),
    input char(20),
    istdat dats(8),
    lenum char(20),
    lgpla char(10),
    lgpla_soll char(10),
    vondat dats(8))
;
CREATE TABLE zsd_one_hub_p1 (
    act_load char(1),
    act_unload char(1),
    aname1 char(35),
    aspdnr char(10),
    colli char(10),
    datum dats(8),
    kfzkz char(10),
    lgtor char(3),
    oname1 char(35),
    ospdnr char(10),
    sammg char(10),
    state string,
    vbeln char(20),
    vtext char(30),
    werks string,
    wname char(30))
;
CREATE TABLE zsd_one_hub_p2 (
    count int2(5),
    gewei unit(3),
    icon_600 char(132),
    icon_status char(132),
    icon_wa char(132),
    loaded int2(5),
    ntgew quan(15),
    open int2(5),
    status_600 char(1),
    status_wa char(1),
    text_wa char(120),
    total int2(5),
    vbeln char(10),
    vkorg char(4),
    vtext char(20))
;
CREATE TABLE zsd_one_hub_p3 (
    anzpa int2(5),
    aufnr char(12),
    bolnr char(35),
    brgew quan(13),
    cload int2(5),
    copen int2(5),
    equnr char(18),
    etikp string,
    gewei unit(3),
    info_txt char(120),
    lenum char(20),
    lesta string,
    lfnum_zdo1 char(10),
    lfnum_zkli char(10),
    lgber_ist char(3),
    lgber_soll char(3),
    lgnum string,
    lgpla_ist char(10),
    lgpla_soll char(10),
    lgtyp_ist string,
    lgtyp_ist_txt char(120),
    lgtyp_soll string,
    pldat dats(8),
    pstat string,
    qmnum char(12),
    reason string,
    sn_profil char(132),
    status_druck char(132),
    status_uml char(132),
    status_we char(132),
    status_zdo1c char(132),
    tanum numc(10),
    vbnum_zdo1 char(10),
    vbnum_zkli char(10),
    vkorg char(4),
    vtext char(20),
    werks char(4),
    westa string,
    zdo1c char(1))
;
CREATE TABLE zsd_one_hub_p4 (
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    bolnr char(8),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    lgtor char(3),
    ulangu lang(1),
    unlst string,
    unlst_icon char(132),
    werks char(4))
;
CREATE TABLE zsd_ord_stat_kon (
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    price_cond char(4) NOT NULL,
    user_session char(40) NOT NULL,
    primary key (price_cond,user_session) )
;
CREATE TABLE zsd_order_simulate_s (
    _color_ table_of_lvc_s_scol,
    _style_ table_of_lvc_s_styl,
    allocated char(1),
    avail_date dats(8),
    avail_qty quan(15),
    avail_type string,
    bom string,
    contingent_qty quan(13),
    delivery_plant char(4),
    hg_lv_item numc(6),
    item_category char(4),
    itm_number numc(6),
    mat_descr char(40),
    material char(18),
    onord_stock quan(13),
    plant char(4),
    purch_group char(3),
    purch_org char(4),
    req_qty quan(15),
    sloc char(4),
    stock quan(13),
    unit unit(3))
;
CREATE TABLE zsd_orderfulfillm (
    auart string,
    aulwe string,
    autlf string,
    bmeng quan(13),
    bstnk char(20),
    edatu dats(8),
    erdat dats(8),
    erzet tims(6),
    etenr numc(4),
    kunnr string,
    kwmeng quan(15),
    lddat dats(8),
    lgort string,
    lifsp string,
    matkl char(9),
    matnr char(18),
    mbdat dats(8),
    mvgr3 string,
    posnr string,
    pstyv string,
    route string,
    spart char(2),
    tddat dats(8),
    vbeln string,
    vdatu dats(8),
    vkorg char(4),
    vrkme string,
    vtweg char(2),
    wadat dats(8),
    werks string,
    zz_fulfilled int4(10),
    zz_intime quan(15),
    zz_notintime quan(15),
    zz_percfulfilled quan(15),
    zz_vrkme1 string,
    zz_vrkme2 string,
    zz_vrkme3 string,
    zzatpstat_date_onb char(10),
    zzatpstat_date_onh dats(8))
;
CREATE TABLE zsd_orderstatusanzeige (
    abgru char(2),
    aedat_rmcode_a dats(8),
    aedat_rmcode_l dats(8),
    aedat_rmcode_z dats(8),
    amount dec(16),
    arktx char(40),
    auart char(4),
    augru char(3),
    beuid char(10),
    bf_leasing_desk char(1),
    bnl_rlist char(1),
    bstnk char(20),
    cnt int4(10),
    deal char(10),
    deldate_plan dats(8),
    duration int4(10),
    ename char(40),
    erdat dats(8),
    ernam char(12),
    finance_data dec(16),
    ifrs dec(16),
    ifrs_rel string,
    inv_dat dats(8),
    invoice_amt dec(16),
    invoice_no char(10),
    kunnr char(10),
    kunwe char(10),
    l_gi_date dats(8),
    lead_source char(20),
    matkl char(9),
    matnr char(18),
    name1 char(30),
    nb_main int1(3),
    ort01 char(35),
    pernr numc(8),
    posnr numc(6),
    prat6 char(1),
    prdha char(18),
    prodh char(18),
    project_order char(24),
    pstlz char(10),
    pstyv char(4),
    qmdab_i dats(8),
    rmcode_a char(3),
    rmcode_l char(3),
    rmcode_z char(3),
    scogf dec(16),
    second_auart char(4),
    second_augru char(3),
    second_order char(10),
    spednr char(10),
    stat char(10),
    stprs dec(16),
    stras char(35),
    third_party char(1),
    used_machine char(1),
    vbeln char(10),
    vertrag char(10),
    vispn char(30),
    vkbur char(4),
    vkgrp char(3),
    vlaufz numc(3),
    vsbed char(2),
    vsty char(38),
    wename char(30),
    wename2 char(30),
    wldat dats(8),
    zda1_amt dec(16),
    zzbbg_partner char(16),
    zzposnr numc(6),
    zzprodarea char(2))
;
CREATE TABLE zsd_ordsim_nocs (
    active char(1) NOT NULL,
    auart char(4) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,vkorg,vtweg) )
;
CREATE TABLE zsd_partn_ecc2s4 (
    ktokd char(4) NOT NULL,
    kunnr char(10) NOT NULL,
    primary key (ktokd,kunnr) )
;
CREATE TABLE zsd_partner_equality_check (
    bzirk_ag string,
    bzirk_delta string,
    bzirk_parvw string,
    ktokd string,
    ktokd2 string,
    kunn2 char(10),
    kunnr char(10),
    name1_ag char(35),
    name1_parvw char(35),
    parvw string,
    pernr numc(8),
    spart string,
    total_delta string,
    vkbur_ag string,
    vkbur_delta string,
    vkbur_parvw string,
    vkgrp_ag string,
    vkgrp_delta string,
    vkgrp_parvw string,
    vkorg string,
    vtweg string)
;
CREATE TABLE zsd_pdi_plant (
    error char(1),
    ktokd char(4),
    kunnr char(10),
    land1 char(3),
    pdi_werk char(4),
    pstlz char(10),
    spart char(2),
    vkorg char(4),
    vtweg char(2),
    vwerk char(4))
;
CREATE TABLE zsd_platform (
    platform char(60) NOT NULL,
    primary key (platform) )
;
CREATE TABLE zsd_plz_de (
    bundesland char(3),
    ort01 char(35) NOT NULL,
    plz char(10) NOT NULL,
    suffix char(35) NOT NULL,
    vorwahl char(16),
    primary key (ort01,plz,suffix) )
;
CREATE TABLE zsd_plz_regions (
    land string NOT NULL,
    plz_bis char(10) NOT NULL,
    plz_von char(10) NOT NULL,
    region string NOT NULL,
    primary key (land,plz_bis,plz_von,region) )
;
CREATE TABLE zsd_plz_regionsc (
    land string NOT NULL,
    primary key (land) )
;
CREATE TABLE zsd_pre_conf_mat (
    auart char(4) NOT NULL,
    matnr char(18) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (auart,matnr,vkorg) )
;
CREATE TABLE zsd_pre_conf_mav (
    auart char(4) NOT NULL,
    matnr char(18) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (auart,matnr,vkorg) )
;
CREATE TABLE zsd_proc_hdr_v (
    add_data char(1300) NOT NULL,
    auart string NOT NULL,
    erdat_va dats(8) NOT NULL,
    ernam_va char(12) NOT NULL,
    erzet_va tims(6) NOT NULL,
    gbstk char(1) NOT NULL,
    global_proc_id char(32) NOT NULL,
    last_step_count int4(10) NOT NULL,
    lock_count numc(10) NOT NULL,
    locked string NOT NULL,
    noc_code string NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    spart string NOT NULL,
    vbeln string NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    primary key (add_data,auart,erdat_va,ernam_va,erzet_va,gbstk,global_proc_id,last_step_count,lock_count,locked,noc_code,proc_code,proc_counter,spart,vbeln,vkorg,vtweg) )
;
CREATE TABLE zsd_proc_setting (
    client char(3) NOT NULL,
    name char(30) NOT NULL,
    value char(1024),
    primary key (client,name) )
;
CREATE TABLE zsd_procexitseq (
    process_id string NOT NULL,
    sequence_id string NOT NULL,
    primary key (process_id,sequence_id) )
;
CREATE TABLE zsd_proj_rep_con (
    erdat dats(8),
    ernam char(12),
    posid char(24),
    post1 char(40),
    prart string,
    prctr string,
    pspnr numc(8) NOT NULL,
    verna char(25),
    primary key (pspnr) )
;
CREATE TABLE zsd_proj_rep_ord (
    erdat dats(8),
    ernam char(12),
    posid char(24),
    post1 char(40),
    prart string,
    prctr string,
    pspnr numc(8) NOT NULL,
    verna char(25),
    zzvkorg char(4),
    primary key (pspnr) )
;
CREATE TABLE zsd_project_reporting (
    zz_lead_source char(20),
    zz_proj_nr char(24),
    zz_proj_valid string)
;
CREATE TABLE zsd_project_reporting_bapevbap (
    zz_enachw char(1),
    zz_enachw_p char(1),
    zz_gutsch char(1),
    zz_lead_source char(20),
    zz_nopra char(1),
    zz_nulrg char(1),
    zz_onere char(1),
    zz_proj_nr char(24),
    zz_proj_valid string,
    zz_sumrg char(1),
    zz_zterm char(4))
;
CREATE TABLE zsd_project_reporting_bapxvbap (
    zz_enachw char(1),
    zz_enachw_p char(1),
    zz_gutsch char(1),
    zz_lead_source char(1),
    zz_nopra char(1),
    zz_nulrg char(1),
    zz_onere char(1),
    zz_proj_nr char(1),
    zz_proj_valid char(1),
    zz_sumrg char(1),
    zz_zterm char(1))
;
CREATE TABLE zsd_project_reporting_vbak (
    zz_lead_source char(20),
    zz_proj_nr char(24),
    zz_proj_valid string)
;
CREATE TABLE zsd_project_reporting_vbap (
    zz_lead_source char(20),
    zz_proj_nr char(24),
    zz_proj_valid string)
;
CREATE TABLE zsd_project_reporting_vbrk (
    zz_proj_contr char(24),
    zz_proj_order char(24))
;
CREATE TABLE zsd_project_reporting_vbrp (
    zz_proj_contr char(24),
    zz_proj_order char(24))
;
CREATE TABLE zsd_project_reportingx (
    zz_lead_source char(1),
    zz_proj_nr char(1),
    zz_proj_valid char(1))
;
CREATE TABLE zsd_projnr_nocs (
    active char(1) NOT NULL,
    auart char(4) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,vkorg,vtweg) )
;
CREATE TABLE zsd_pstyv_excl (
    auart string NOT NULL,
    pstyv string NOT NULL,
    spart string NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    primary key (auart,spart,vkorg,vtweg) )
;
CREATE TABLE zsd_range_bstkd (
    high char(35),
    low char(35),
    option string,
    sign string)
;
CREATE TABLE zsd_range_bstnk_s (
    high char(20),
    low char(20),
    option string,
    sign string)
;
CREATE TABLE zsd_range_bzirk_s (
    high char(6),
    low char(6),
    option string,
    sign string)
;
CREATE TABLE zsd_range_ekgrp_s (
    high char(3),
    low char(3),
    option string,
    sign string)
;
CREATE TABLE zsd_range_erdat_s (
    high dats(8),
    low dats(8),
    option string,
    sign string)
;
CREATE TABLE zsd_range_ernam_s (
    high char(12),
    low char(12),
    option string,
    sign string)
;
CREATE TABLE zsd_range_gbstk_s (
    high char(1),
    low char(1),
    option string,
    sign string)
;
CREATE TABLE zsd_range_matkl_s (
    high char(9),
    low char(9),
    option string,
    sign string)
;
CREATE TABLE zsd_range_matnr_s (
    high char(18),
    low char(18),
    option string,
    sign string)
;
CREATE TABLE zsd_range_msgty_s (
    high string,
    low string,
    option string,
    sign string)
;
CREATE TABLE zsd_range_proj_s (
    high char(24),
    low char(24),
    option string,
    sign string)
;
CREATE TABLE zsd_range_spart_s (
    high char(2),
    low char(2),
    option string,
    sign string)
;
CREATE TABLE zsd_range_vdatu_s (
    high dats(8),
    low dats(8),
    option string,
    sign string)
;
CREATE TABLE zsd_range_vkbur_s (
    high char(4),
    low char(4),
    option string,
    sign string)
;
CREATE TABLE zsd_range_vkgrp_s (
    high char(3),
    low char(3),
    option string,
    sign string)
;
CREATE TABLE zsd_range_vkorg_s (
    high char(4),
    low char(4),
    option string,
    sign string)
;
CREATE TABLE zsd_range_vsbed_s (
    high char(2),
    low char(2),
    option string,
    sign string)
;
CREATE TABLE zsd_range_vtweg_s (
    high char(2),
    low char(2),
    option string,
    sign string)
;
CREATE TABLE zsd_responsible_ext (
    zzresponsible char(3))
;
CREATE TABLE zsd_responsible_ext_oz (
    zzresponsible char(3))
;
CREATE TABLE zsd_responsible_extension (
    zzresponsible char(3))
;
CREATE TABLE zsd_responsible_extension_bapx (
    zzresponsible char(1))
;
CREATE TABLE zsd_responsible_extension_ozx (
    zzresponsible char(1))
;
CREATE TABLE zsd_responsible_extensionx (
    zzresponsible char(1))
;
CREATE TABLE zsd_ret01_action (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    endda dats(8),
    event string,
    proc_mode string,
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_ret01_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    ref_data char(30),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ret01_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ret01_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_ret01_st_t (
    bezei char(50) NOT NULL,
    spras string NOT NULL,
    step string NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_ret01_st_v (
    bezei char(50),
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_ret01_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ret02_action (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    endda dats(8),
    event string,
    proc_mode string,
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_ret02_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    ref_data char(30),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ret02_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ret02_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_ret02_st_t (
    bezei char(50) NOT NULL,
    spras string NOT NULL,
    step string NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_ret02_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_ret_bom_mat (
    auart char(4) NOT NULL,
    matnr char(18),
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,spart,vkorg,vtweg) )
;
CREATE TABLE zsd_retour_s (
    arktx char(40),
    comm_type string,
    csrc_managed char(1),
    equnr char(18),
    erdat dats(8),
    kunnr char(10),
    kunnr_akt char(10),
    matnr char(18),
    posnr numc(6),
    vbeln char(10))
;
CREATE TABLE zsd_return_email (
    auart char(4) NOT NULL,
    augru char(3) NOT NULL,
    email char(241),
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,augru,spart,vkorg,vtweg) )
;
CREATE TABLE zsd_rfc_vbfa_s (
    posnn numc(6),
    posnv numc(6),
    stufe numc(2),
    syst_n char(8),
    syst_v char(8),
    vbeln char(10),
    vbelv char(10),
    vbtyp_n string,
    vbtyp_v string)
;
CREATE TABLE zsd_right_to_benefit (
    zsd_rtb string)
;
CREATE TABLE zsd_routen_cus1 (
    kunag string NOT NULL,
    kunwe string NOT NULL,
    land1 string NOT NULL,
    specialroute numc(8) NOT NULL,
    vkorg string NOT NULL,
    vsbed string NOT NULL,
    vstel string NOT NULL,
    vtweg string NOT NULL,
    primary key (kunag,kunwe,land1,vkorg,vsbed,vstel,vtweg) )
;
CREATE TABLE zsd_routen_cus2 (
    gewicht_bis quan(13) NOT NULL,
    land1 string NOT NULL,
    meins string,
    parnr_sp string,
    parnr_sr string,
    route string,
    specialroute numc(8) NOT NULL,
    vkorg string NOT NULL,
    vsbed string,
    primary key (gewicht_bis,land1,specialroute,vkorg) )
;
CREATE TABLE zsd_routen_cusfr (
    auart string NOT NULL,
    gewicht_bis quan(13) NOT NULL,
    land1 string NOT NULL,
    meins unit(3),
    parnr_sp char(10) NOT NULL,
    parnr_sr char(10),
    route string,
    vkbur string NOT NULL,
    vkorg string NOT NULL,
    vsbed string,
    vstel string NOT NULL,
    vtweg string NOT NULL,
    primary key (auart,gewicht_bis,land1,parnr_sp,vkbur,vkorg,vstel,vtweg) )
;
CREATE TABLE zsd_routen_exep (
    kunnr char(10) NOT NULL,
    vkorg char(4) NOT NULL,
    vsbed char(2) NOT NULL,
    vstel char(4) NOT NULL,
    primary key (kunnr,vkorg,vsbed,vstel) )
;
CREATE TABLE zsd_routen_lgort (
    auart string NOT NULL,
    land1 string NOT NULL,
    lgort char(4) NOT NULL,
    matnr char(18) NOT NULL,
    route string NOT NULL,
    specialroute string NOT NULL,
    vkorg string NOT NULL,
    werks string NOT NULL,
    primary key (auart,land1,lgort,matnr,route,vkorg,werks) )
;
CREATE TABLE zsd_rtb_append (
    zsd_rtb string)
;
CREATE TABLE zsd_rtb_append2 (
    zsd_rtb string)
;
CREATE TABLE zsd_s_app_hkunnr (
    zz_bsark char(4),
    zz_hkunnr_level_a char(10),
    zz_hkunnr_level_b char(10),
    zz_hkunnr_level_c char(10),
    zz_hkunnr_level_d char(10),
    zz_hkunnr_level_e char(10))
;
CREATE TABLE zsd_s_assigned_equi (
    addr_group char(4),
    addrnumber char(10),
    building char(20),
    city1 char(40),
    country char(3),
    datab dats(8),
    datbi dats(8),
    equi_ag numc(10),
    equi_re numc(10),
    equi_rg numc(10),
    equi_we numc(10),
    equnr char(18),
    fax_extens char(10),
    fax_number char(30),
    floor char(10),
    house_num1 char(10),
    langu lang(1),
    matnr char(18),
    name1 char(40),
    name2 char(40),
    name3 char(40),
    name4 char(40),
    name_co char(40),
    objnr char(22),
    posnr numc(6),
    post_code1 char(10),
    region char(3),
    remark char(50),
    roomnumber char(10),
    sernr char(18),
    smtp_addr char(241),
    sort1 char(20),
    sort2 char(20),
    street char(60),
    tel_extens char(10),
    tel_number char(30),
    title char(4),
    vbeln char(10))
;
CREATE TABLE zsd_s_billplan_alv (
    afdat dats(8),
    celltab table_of_lvc_s_styl,
    col01 char(25),
    col02 char(40),
    col03 char(40),
    col04 char(25),
    col05 char(25),
    fakwr curr(15),
    fkmng quan(13),
    fksaf char(1),
    fplnr char(10),
    fpltr numc(6),
    kwmeng quan(15),
    linecolor char(4),
    meins unit(3),
    posnr numc(6),
    waers cuky(5))
;
CREATE TABLE zsd_s_ce (
    autte char(1),
    bedat dats(8),
    fplnr char(10),
    peraf char(2),
    perio char(2),
    posnr numc(6),
    vbegdat dats(8),
    vbeln char(10),
    vdemdat dats(8),
    venddat dats(8),
    vlaufz numc(3),
    zautte char(1),
    zbedat dats(8),
    zperaf char(2),
    zperio char(2))
;
CREATE TABLE zsd_s_check_last_confirm_date (
    cdate dats(8),
    name_first char(40),
    name_last char(40),
    pernr numc(8))
;
CREATE TABLE zsd_s_csv (
    lastl char(1),
    nr_1 char(120),
    nr_10 char(120),
    nr_11 char(120),
    nr_12 char(120),
    nr_13 char(120),
    nr_14 char(120),
    nr_15 char(120),
    nr_16 char(120),
    nr_17 char(120),
    nr_18 char(120),
    nr_19 char(120),
    nr_2 char(120),
    nr_20 char(120),
    nr_21 char(120),
    nr_22 char(120),
    nr_23 char(120),
    nr_24 char(120),
    nr_25 char(120),
    nr_3 char(120),
    nr_4 char(120),
    nr_5 char(120),
    nr_6 char(120),
    nr_7 char(120),
    nr_8 char(120),
    nr_9 char(120))
;
CREATE TABLE zsd_s_cust_inact_out (
    ag char(10),
    aufsd char(2),
    aufsdvert char(2),
    beuid char(10),
    bran1 char(10),
    bukrs char(4),
    busab char(2),
    bzirk char(6),
    datlt char(14),
    dsoin numc(4),
    endda dats(8),
    grupp char(4),
    hkunnr char(10),
    hkunnr_kh char(10),
    info char(50),
    konzs char(10),
    ktokd char(4),
    kunnr char(10),
    land1 char(3),
    loevert char(1),
    loevm char(1),
    mansp char(1),
    message char(50),
    name char(30),
    name1 char(35),
    name2 char(35),
    name_ys char(30),
    name_yy char(30),
    name_z6 char(30),
    name_zm char(30),
    ort01 char(35),
    pstlz char(10),
    sbgrp char(3),
    spart char(2),
    stras char(35),
    ve numc(8),
    vkbur char(4),
    vkgrp char(3),
    vkorg char(4),
    vtweg char(2),
    ys numc(8),
    yy numc(8),
    z6 numc(8),
    zb char(10),
    zg char(10),
    zm numc(8),
    zzconsum char(1))
;
CREATE TABLE zsd_s_del_batch (
    charg char(10),
    matnr char(18),
    posnr numc(6),
    pstyv char(4),
    vbeln char(10))
;
CREATE TABLE zsd_s_delivery (
    absta char(1),
    abstk char(1),
    besta char(1),
    bestk char(1),
    buchk char(1),
    charg char(10),
    erdat dats(8),
    fksaa char(1),
    fksak char(1),
    fksta char(1),
    fkstk char(1),
    gbsta char(1),
    gbstk char(1),
    invcount int2(5),
    invstat char(1),
    kosta char(1),
    kostk char(1),
    lfart char(4),
    lfgsa char(1),
    lfgsk char(1),
    lfimg quan(13),
    lfsta char(1),
    lfstk char(1),
    lgort char(4),
    lvsta char(1),
    lvstk char(1),
    matkl char(9),
    matnr char(18),
    matwa char(18),
    meins unit(3),
    posnr numc(6),
    posnrli numc(6),
    prodh char(18),
    ref_posnrli numc(6),
    ref_vbelnli char(10),
    route char(6),
    total_dlv_stat string,
    uvall char(1),
    vbeln char(10),
    vbelnli char(10),
    vkorg char(4),
    vrkme unit(3),
    vstel char(4),
    wadat dats(8),
    wbsta char(1),
    wbstk char(1),
    werks char(4))
;
CREATE TABLE zsd_s_delivery_dt1 (
    abstadt1 char(1),
    abstkdt1 char(1),
    bestadt1 char(1),
    bestkdt1 char(1),
    buchkdt1 char(1),
    chargdt1 char(10),
    erdatdt1 dats(8),
    fksaadt1 char(1),
    fksakdt1 char(1),
    fkstadt1 char(1),
    fkstkdt1 char(1),
    gbstadt1 char(1),
    gbstkdt1 char(1),
    invcountdt1 int2(5),
    invstatdt1 char(1),
    kostadt1 char(1),
    kostkdt1 char(1),
    lfartdt1 char(4),
    lfgsadt1 char(1),
    lfgskdt1 char(1),
    lfimgdt1 quan(13),
    lfstadt1 char(1),
    lfstkdt1 char(1),
    lgortdt1 char(4),
    lvstadt1 char(1),
    lvstkdt1 char(1),
    matkldt1 char(9),
    matnrdt1 char(18),
    matwadt1 char(18),
    meinsdt1 unit(3),
    posnrdt1 numc(6),
    posnrlidt1 numc(6),
    prodhdt1 char(18),
    ref_posnrlidt1 numc(6),
    ref_vbelnlidt1 char(10),
    routedt1 char(6),
    total_dlv_statdt1 string,
    uvalldt1 char(1),
    vbelndt1 char(10),
    vbelnlidt1 char(10),
    vkorgdt1 char(4),
    vrkmedt1 unit(3),
    vsteldt1 char(4),
    wadatdt1 dats(8),
    wbstadt1 char(1),
    wbstkdt1 char(1),
    werksdt1 char(4))
;
CREATE TABLE zsd_s_elo_delivery (
    vbeln char(10))
;
CREATE TABLE zsd_s_elo_ztm_data (
    contract_item_1 numc(6),
    contract_item_2 numc(6),
    contract_numb_1 char(10),
    contract_numb_2 char(10),
    dali string,
    equipment char(18),
    kunnr char(10),
    linking_pin char(60),
    vbeln char(10),
    vbeln_vl char(10))
;
CREATE TABLE zsd_s_hkunnr (
    zz_bsark char(4),
    zz_hkunnr_level_a char(10),
    zz_hkunnr_level_b char(10),
    zz_hkunnr_level_c char(10),
    zz_hkunnr_level_d char(10),
    zz_hkunnr_level_e char(10))
;
CREATE TABLE zsd_s_import_wph (
    contract_id char(10),
    country char(3),
    equnr char(20),
    license_created char(14),
    license_expires char(14),
    license_name char(100),
    license_type char(20),
    matnr char(18),
    record_id char(3),
    serialnr char(72),
    ship_to char(10),
    sold_to char(10),
    timestamp char(14),
    value_current int4(10),
    value_max int4(10),
    value_previous int4(10))
;
CREATE TABLE zsd_s_invoice (
    arktx char(40),
    erdat dats(8),
    fkart char(4),
    fkdat dats(8),
    fkimg quan(13),
    fproz dec(5),
    invcount int2(5),
    invstat char(1),
    matnr char(18),
    mwsbp curr(13),
    netwr curr(15),
    posnr numc(6),
    posnvi numc(6),
    ref_posnvi numc(6),
    ref_vbelni char(10),
    total_inv_stat string,
    vbeln char(10),
    vbelni char(10),
    vbtyp_n char(1),
    vrkme unit(3),
    waerk cuky(5))
;
CREATE TABLE zsd_s_invoice_it1 (
    arktxit1 char(40),
    erdatit1 dats(8),
    fkartit1 char(4),
    fkdatit1 dats(8),
    fkimgit1 quan(13),
    fprozit1 dec(5),
    invcountit1 int2(5),
    invstatit1 char(1),
    matnrit1 char(18),
    mwsbpit1 curr(13),
    netwrit1 curr(15),
    posnrit1 numc(6),
    posnviit1 numc(6),
    ref_posnviit1 numc(6),
    ref_vbelniit1 char(10),
    total_inv_statit1 string,
    vbelniit1 char(10),
    vbelnit1 char(10),
    vbtyp_nit1 char(1),
    vrkmeit1 unit(3),
    waerkit1 cuky(5))
;
CREATE TABLE zsd_s_order_dp (
    augbl char(10),
    augdt dats(8),
    belnr char(10),
    bukrs char(4),
    dmbrt curr(13),
    gjahr numc(4),
    mwsts curr(13),
    netwr curr(15),
    vbeln char(10),
    waers cuky(5))
;
CREATE TABLE zsd_s_orderstat_dat (
    abgru char(2),
    absta char(1),
    absta2 char(1),
    abstadt1 char(1),
    abstk char(1),
    abstk2 char(1),
    abstkdt1 char(1),
    aedat dats(8),
    arktx char(40),
    arktxit1 char(40),
    arktxt2 char(40),
    auart char(4),
    auartt2 char(4),
    augru char(3),
    banfn char(10),
    besta char(1),
    besta2 char(1),
    bestadt1 char(1),
    bestk char(1),
    bestk2 char(1),
    bestkdt1 char(1),
    bezei_vkbur char(20),
    bf_leasing_desk char(1),
    bnfpo numc(5),
    brtwr curr(13),
    bsart char(4),
    bstkd char(35),
    bstyp string,
    buchk char(1),
    buchk2 char(1),
    buchkdt1 char(1),
    bukrs char(4),
    chargdt1 char(10),
    diff_quan_po quan(13),
    diff_val_po curr(13),
    ebeln char(10),
    ebelp numc(5),
    ekgrp char(3),
    equnr char(18),
    erdat dats(8),
    erdatdt1 dats(8),
    erdatit1 dats(8),
    ernam char(12),
    fkartit1 char(4),
    fkdatit1 dats(8),
    fkimgit1 quan(13),
    fksaa char(1),
    fksaa2 char(1),
    fksaadt1 char(1),
    fksak char(1),
    fksak2 char(1),
    fksakdt1 char(1),
    fksta char(1),
    fksta2 char(1),
    fkstadt1 char(1),
    fkstk char(1),
    fkstk2 char(1),
    fkstkdt1 char(1),
    fprozit1 dec(5),
    gbsta char(1),
    gbsta2 char(1),
    gbstadt1 char(1),
    gbstk char(1),
    gbstk2 char(1),
    gbstkdt1 char(1),
    idnlf char(35),
    invcountdt1 int2(5),
    invcountit1 int2(5),
    invstatdt1 char(1),
    invstatit1 char(1),
    kappl char(2),
    kbetr curr(11),
    kbmeng quan(15),
    kkurs dec(9),
    kmein unit(3),
    knumv char(10),
    kosta char(1),
    kosta2 char(1),
    kostadt1 char(1),
    kostk char(1),
    kostk2 char(1),
    kostkdt1 char(1),
    kpein dec(5),
    kposn numc(6),
    krech string,
    kschl char(4),
    ktmng quan(13),
    kunnr char(10),
    kwmeng quan(15),
    kwmengt2 quan(15),
    lfartdt1 char(4),
    lfgsa char(1),
    lfgsa2 char(1),
    lfgsadt1 char(1),
    lfgsk char(1),
    lfgsk2 char(1),
    lfgskdt1 char(1),
    lfimgdt1 quan(13),
    lfsta char(1),
    lfsta2 char(1),
    lfstadt1 char(1),
    lfstk char(1),
    lfstk2 char(1),
    lfstkdt1 char(1),
    lgort char(4),
    lgortdt1 char(4),
    lifnr char(10),
    loekzh char(1),
    loekzi char(1),
    lsmng quan(13),
    lvsta char(1),
    lvsta2 char(1),
    lvstadt1 char(1),
    lvstk char(1),
    lvstk2 char(1),
    lvstkdt1 char(1),
    matkl char(9),
    matkldt1 char(9),
    matnr char(18),
    matnrdt1 char(18),
    matnrit1 char(18),
    matnrpo char(18),
    matnrt2 char(18),
    matwadt1 char(18),
    meins unit(3),
    meinsdt1 unit(3),
    menge quan(13),
    mwsbpit1 curr(13),
    name1 char(35),
    netwr curr(15),
    netwr2 curr(15),
    netwrit1 curr(15),
    payer char(10),
    payer_name char(35),
    posnr numc(6),
    posnrdt1 numc(6),
    posnrit1 numc(6),
    posnrlidt1 numc(6),
    posnrt2 numc(6),
    posnv2 numc(6),
    posnviit1 numc(6),
    posnvpo numc(5),
    post1 char(40),
    post12 char(40),
    prat6 char(1),
    prdha char(18),
    prdhat2 char(18),
    prodh char(18),
    prodhdt1 char(18),
    ps_psp_pnr numc(8),
    ps_psp_pnr2 numc(8),
    psphi numc(8),
    psphi2 numc(8),
    pspnr2 char(8),
    pspnr_int char(8),
    pstyv char(4),
    pstyvt2 char(4),
    quan_po quan(13),
    reewr curr(13),
    ref_posnrlidt1 numc(6),
    ref_posnv2 numc(6),
    ref_posnviit1 numc(6),
    ref_posnvpo numc(6),
    ref_vbeln2 char(10),
    ref_vbelniit1 char(10),
    ref_vbelnlidt1 char(10),
    ref_vbelnpo char(10),
    respper numc(8),
    routedt1 char(6),
    salesp numc(8),
    sname char(30),
    spart char(2),
    spartt2 char(2),
    stunr numc(3),
    submi char(10),
    submit2 char(10),
    total_dlv_statdt1 string,
    total_inv_statit1 string,
    total_purch_stat string,
    total_status string,
    txz01 char(40),
    uepos numc(6),
    uvall char(1),
    uvall2 char(1),
    uvall_uk char(1),
    uvalldt1 char(1),
    uvals char(1),
    uvfak char(1),
    uvfas char(1),
    uvprs char(1),
    uvvlk char(1),
    uvvls char(1),
    val_po curr(13),
    vbeln char(10),
    vbeln2 char(10),
    vbelndt1 char(10),
    vbelniit1 char(10),
    vbelnit1 char(10),
    vbelnlidt1 char(10),
    vbelnpo char(10),
    vbelnt2 char(10),
    vbtyp string,
    vbtyp_nit1 char(1),
    vbtypt2 string,
    vkbur char(4),
    vkgrp char(3),
    vkorg char(4),
    vkorgdt1 char(4),
    vkorgt2 char(4),
    vrkme unit(3),
    vrkmedt1 unit(3),
    vrkmeit1 unit(3),
    vrkmet2 unit(3),
    vsteldt1 char(4),
    vtweg char(2),
    vtwegt2 char(2),
    wadatdt1 dats(8),
    waerk2 cuky(5),
    waerkit1 cuky(5),
    waers_konv cuky(5),
    waers_po cuky(5),
    wbsta char(1),
    wbsta2 char(1),
    wbstadt1 char(1),
    wbstk char(1),
    wbstk2 char(1),
    wbstkdt1 char(1),
    werks char(4),
    werksdt1 char(4),
    werkspo char(4),
    werkst2 char(4),
    zaehk numc(2),
    zzitbmprojektname char(40),
    zzitbmprojektname2 char(40),
    zzprodarea char(2),
    zzprodareat2 char(2))
;
CREATE TABLE zsd_s_orderstatus_first_data (
    abgru char(2),
    absta char(1),
    abstk char(1),
    arktx char(40),
    auart char(4),
    augru char(3),
    besta char(1),
    bestk char(1),
    bezei_vkbur char(20),
    bf_leasing_desk char(1),
    bstkd char(35),
    buchk char(1),
    equnr char(18),
    erdat dats(8),
    ernam char(12),
    fksaa char(1),
    fksak char(1),
    fksta char(1),
    fkstk char(1),
    gbsta char(1),
    gbstk char(1),
    kappl char(2),
    kbetr curr(11),
    kbmeng quan(15),
    kkurs dec(9),
    kmein unit(3),
    knumv char(10),
    kosta char(1),
    kostk char(1),
    kpein dec(5),
    kposn numc(6),
    krech string,
    kschl char(4),
    kunnr char(10),
    kwmeng quan(15),
    lfgsa char(1),
    lfgsk char(1),
    lfsta char(1),
    lfstk char(1),
    lvsta char(1),
    lvstk char(1),
    matkl char(9),
    matnr char(18),
    name1 char(35),
    netwr curr(15),
    payer char(10),
    payer_name char(35),
    posnr numc(6),
    post1 char(40),
    prat6 char(1),
    prdha char(18),
    prodh char(18),
    ps_psp_pnr numc(8),
    psphi numc(8),
    pspnr_int char(8),
    pstyv char(4),
    respper numc(8),
    salesp numc(8),
    sname char(30),
    spart char(2),
    stunr numc(3),
    submi char(10),
    total_status string,
    uepos numc(6),
    uvall char(1),
    uvall_uk char(1),
    uvals char(1),
    uvfak char(1),
    uvfas char(1),
    uvprs char(1),
    uvvlk char(1),
    uvvls char(1),
    vbeln char(10),
    vbtyp string,
    vkbur char(4),
    vkgrp char(3),
    vkorg char(4),
    vrkme unit(3),
    vtweg char(2),
    waers_konv cuky(5),
    wbsta char(1),
    wbstk char(1),
    werks char(4),
    zaehk numc(2),
    zzitbmprojektname char(40),
    zzprodarea char(2))
;
CREATE TABLE zsd_s_popup_alv (
    afdat dats(8),
    arbqa quan(15),
    arktx char(40),
    billq quan(15),
    celltab table_of_lvc_s_styl,
    kwmeng quan(15),
    matnr char(18),
    netpr curr(11),
    posnr numc(6),
    tover char(1),
    vbeln char(10),
    vrkme unit(3),
    waerk cuky(5),
    zz_netwr curr(15))
;
CREATE TABLE zsd_s_price_cond (
    kappl char(2),
    kbetr curr(11),
    kkurs dec(9),
    kmein unit(3),
    knumv char(10),
    kpein dec(5),
    kposn numc(6),
    krech string,
    kschl char(4),
    stunr numc(3),
    waers_konv cuky(5),
    zaehk numc(2))
;
CREATE TABLE zsd_s_purchase_order (
    aedat dats(8),
    banfn char(10),
    bnfpo numc(5),
    brtwr curr(13),
    bsart char(4),
    bstyp string,
    bukrs char(4),
    diff_quan_po quan(13),
    diff_val_po curr(13),
    ebeln char(10),
    ebelp numc(5),
    ekgrp char(3),
    idnlf char(35),
    ktmng quan(13),
    lgort char(4),
    lifnr char(10),
    loekzh char(1),
    loekzi char(1),
    lsmng quan(13),
    matnrpo char(18),
    meins unit(3),
    menge quan(13),
    posnvpo numc(5),
    quan_po quan(13),
    reewr curr(13),
    ref_posnvpo numc(6),
    ref_vbelnpo char(10),
    total_purch_stat string,
    txz01 char(40),
    val_po curr(13),
    vbelnpo char(10),
    waers_po cuky(5),
    werkspo char(4))
;
CREATE TABLE zsd_s_rep_vbacva05 (
    arktx char(40),
    auart char(4),
    bstkd char(35),
    charg char(10),
    edatu dats(8),
    erdat dats(8),
    ernam char(12),
    katr3 char(2),
    kdmat char(35),
    kunnr char(10),
    kunwe char(10),
    kwmeng quan(15),
    labst quan(13),
    lfsta char(1),
    matnr char(18),
    mtart char(4),
    name1 char(35),
    omeng quan(15),
    posnr numc(6),
    vbeln char(10),
    vdatu dats(8),
    vkbur char(4),
    vkorg char(4),
    vrkme unit(3),
    vtext char(20),
    vtweg char(2),
    wadat dats(8),
    werks char(4),
    zzatcmgst char(20),
    zzatlifsk char(20),
    zzatpqty_onb quan(13),
    zzatpqty_onh quan(13),
    zzatpstat_date_onb char(10),
    zzatpstat_date_onh dats(8),
    zzprodarea char(2))
;
CREATE TABLE zsd_s_second_order (
    absta2 char(1),
    abstk2 char(1),
    arktxt2 char(40),
    auartt2 char(4),
    besta2 char(1),
    bestk2 char(1),
    buchk2 char(1),
    fksaa2 char(1),
    fksak2 char(1),
    fksta2 char(1),
    fkstk2 char(1),
    gbsta2 char(1),
    gbstk2 char(1),
    kosta2 char(1),
    kostk2 char(1),
    kwmengt2 quan(15),
    lfgsa2 char(1),
    lfgsk2 char(1),
    lfsta2 char(1),
    lfstk2 char(1),
    lvsta2 char(1),
    lvstk2 char(1),
    matnrt2 char(18),
    netwr2 curr(15),
    posnrt2 numc(6),
    posnv2 numc(6),
    post12 char(40),
    prdhat2 char(18),
    ps_psp_pnr2 numc(8),
    psphi2 numc(8),
    pspnr2 char(8),
    pstyvt2 char(4),
    ref_posnv2 numc(6),
    ref_vbeln2 char(10),
    spartt2 char(2),
    submit2 char(10),
    uvall2 char(1),
    vbeln2 char(10),
    vbelnt2 char(10),
    vbtypt2 string,
    vkorgt2 char(4),
    vrkmet2 unit(3),
    vtwegt2 char(2),
    waerk2 cuky(5),
    wbsta2 char(1),
    wbstk2 char(1),
    werkst2 char(4),
    zzitbmprojektname2 char(40),
    zzprodareat2 char(2))
;
CREATE TABLE zsd_s_tmoec (
    action string,
    action_cpi char(4),
    action_ubp char(4),
    action_wf2 char(4),
    action_z2f char(4),
    action_z2l char(4),
    aedat_rmcode_z dats(8),
    arktx char(40),
    auart char(4),
    bf_leasing_desk char(1),
    bill_lf char(10),
    bill_ord char(10),
    bstkd char(35),
    cascading curr(13),
    comment strg(0),
    contract char(10),
    contract_item numc(6),
    contract_type string,
    contract_type_t char(30),
    costs_of_goods curr(13),
    ddtext char(60),
    dlv_fkstk char(1),
    dlv_gbstk char(1),
    erdat_va dats(8),
    erekz_cnt int4(10),
    ernam string,
    errortype char(20),
    exidv2 char(20),
    filter_control char(1),
    fkrel_f int4(10),
    fkrel_g int4(10),
    fksak char(1),
    fully_inv char(1),
    gbstk char(1),
    gvfall string,
    internal_id numc(8),
    invl_lf char(10),
    invl_ord char(10),
    item_comp int4(10),
    kbetr_sum curr(11),
    kbetr_zpd curr(11),
    kbetr_zsw curr(11),
    kunnr char(10),
    kwaeh cuky(5),
    kz_nullpr char(1),
    lfgsk char(1),
    lifsk char(2),
    lt_vbeln_vl table_of_vbeln_vl,
    ltext char(80),
    matkl char(9),
    matnr char(18),
    na_vstat string,
    name1 char(35),
    netwr curr(15),
    not_relevant char(1),
    objnr char(22),
    oenum char(10),
    pernr numc(8),
    po_avail char(1),
    prodh char(18),
    pstyv_i int4(10),
    pstyv_zdi1 int4(10),
    pstyv_zds2 int4(10),
    pstyv_zdtp int4(10),
    repos_cnt int4(10),
    rlist char(1),
    rmcode_z char(3),
    sec_ord string,
    spart string,
    stat char(5),
    stat_deal char(5),
    stat_deal_closed char(1),
    stat_invr char(5),
    stat_order char(5),
    trref_l char(10),
    turnover curr(13),
    uvall char(1),
    vbeln_s char(10),
    vbeln_va char(10),
    vbeln_vl char(10),
    vbeln_vl_beu char(10),
    vbtyp string,
    vdatu dats(8),
    vkbur char(4),
    vkgrp char(3),
    vkorg string,
    vtweg string,
    waerk cuky(5),
    waers_ca cuky(5),
    waers_cog cuky(5),
    waers_to cuky(5),
    werks char(4),
    zm_parnter char(25),
    zm_partner_name char(40),
    zzprodarea char(2))
;
CREATE TABLE zsd_s_tmoec_pstyv (
    fkrel_f int4(10),
    fkrel_g int4(10),
    item_comp int4(10),
    pstyv_i int4(10),
    pstyv_zdi1 int4(10),
    pstyv_zds2 int4(10),
    pstyv_zdtp int4(10))
;
CREATE TABLE zsd_s_vbap (
    kwmeng quan(15),
    matnr char(18),
    posnr numc(6),
    vbeln char(10),
    vrkme unit(3),
    werks char(4))
;
CREATE TABLE zsd_sbb_ind_equi (
    datab dats(8),
    datbi dats(8),
    equnr char(18) NOT NULL,
    kunag char(10) NOT NULL,
    kunwe char(10) NOT NULL,
    kwert curr(13),
    land1 char(3),
    last_inv_date dats(8),
    matnr char(18) NOT NULL,
    meins unit(3),
    menge quan(13),
    termination char(1),
    vkorg char(4),
    waers cuky(5),
    primary key (equnr,kunag,kunwe,matnr) )
;
CREATE TABLE zsd_sbb_ind_equi_s (
    chg_ind string,
    datab dats(8),
    datbi dats(8),
    equnr char(18),
    kunag char(10),
    kunwe char(10),
    kwert curr(13),
    land1 char(3),
    last_inv_date dats(8),
    matnr char(18),
    meins unit(3),
    menge quan(13),
    termination char(1),
    vkorg char(4),
    waers cuky(5))
;
CREATE TABLE zsd_sbb_quantiti (
    abrechnungsmenge quan(13),
    equipmentnummer char(18) NOT NULL,
    gueltig_ab dats(8),
    gueltig_bis dats(8),
    konditionswert curr(13),
    letztes_abrechnungsdatum dats(8),
    material char(18) NOT NULL,
    mengeneinheit unit(3),
    terminierungskennzeichen char(1),
    waehrung cuky(5),
    primary key (equipmentnummer,material) )
;
CREATE TABLE zsd_send_type (
    sendt char(20) NOT NULL,
    primary key (sendt) )
;
CREATE TABLE zsd_source_det (
    auart string NOT NULL,
    dwerk string NOT NULL,
    dwfix char(1) NOT NULL,
    dwfum char(1) NOT NULL,
    dwhie char(1) NOT NULL,
    dwmvk char(1) NOT NULL,
    matkl string NOT NULL,
    mtpos char(4) NOT NULL,
    rwerk char(1) NOT NULL,
    spart string NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    weight char(1) NOT NULL,
    primary key (auart,matkl,mtpos,spart,vkorg,vtweg) )
;
CREATE TABLE zsd_source_determine (
    auart char(4),
    hvbap_dwerk char(4),
    knmt__dwerk char(4),
    kunnr char(10),
    kunnv char(10),
    kunwe char(10),
    kuwev_dwerk char(4),
    kwmeng quan(15),
    land1 char(3),
    maapv_dwerk char(4),
    matkl char(9),
    matnr char(18),
    matnv char(18),
    mtpos char(4),
    parea char(2),
    pcode char(18),
    prodh char(18),
    pstlz char(10),
    rv45a_dwerk char(4),
    spart char(2),
    uepmt char(4),
    uepos numc(6),
    uepst char(4),
    vkbur char(4),
    vkgrp char(3),
    vkorg char(4),
    vrkme unit(3),
    vtweg char(2),
    vwpos char(4))
;
CREATE TABLE zsd_source_hie (
    charg string NOT NULL,
    dwerk string NOT NULL,
    kunnr char(10) NOT NULL,
    kunwe char(10) NOT NULL,
    lfdnr char(3) NOT NULL,
    lgort string NOT NULL,
    matkl string NOT NULL,
    pstyv char(4),
    spart string NOT NULL,
    vfppr char(1) NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    primary key (kunnr,kunwe,lfdnr,matkl,spart,vkorg,vtweg) )
;
CREATE TABLE zsd_source_lgo (
    auart string NOT NULL,
    dwerk string NOT NULL,
    kunnr string NOT NULL,
    kunwe string NOT NULL,
    lfdnr char(3) NOT NULL,
    lgort string NOT NULL,
    matkl string NOT NULL,
    mtpos string NOT NULL,
    pstyv string NOT NULL,
    spart string NOT NULL,
    uepmt string NOT NULL,
    vfppr char(1) NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    primary key (auart,dwerk,kunnr,kunwe,lfdnr,matkl,mtpos,spart,uepmt,vkorg,vtweg) )
;
CREATE TABLE zsd_source_prodh (
    prodh string NOT NULL,
    vkorg string NOT NULL,
    primary key (prodh,vkorg) )
;
CREATE TABLE zsd_source_wght (
    hilv_ind char(1),
    lgort char(4),
    matkl char(9) NOT NULL,
    pstyv char(4),
    spart char(2) NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    weight_ind char(2) NOT NULL,
    weight_limit quan(13) NOT NULL,
    weight_unit unit(3),
    werks char(4),
    primary key (matkl,spart,vkorg,vtweg,weight_ind,weight_limit) )
;
CREATE TABLE zsd_ss_miniva41 (
    bemerkung char(80),
    equnr char(18),
    kunnr char(10),
    kunzd char(10),
    posnr_va_old numc(6),
    vbeln_va_new char(10),
    vbeln_va_old char(10),
    vdat char(10),
    waers cuky(5),
    zins curr(11),
    zsep curr(11))
;
CREATE TABLE zsd_ss_zklk2zklr_batchdata (
    batch char(10),
    material char(18),
    plant char(4),
    plant_old char(4))
;
CREATE TABLE zsd_struct_range_c10 (
    high char(10),
    low char(10),
    option string,
    sign string)
;
CREATE TABLE zsd_sw_license (
    item char(50),
    item_code char(20),
    token char(50))
;
CREATE TABLE zsd_swrel_action (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    endda dats(8),
    event char(32),
    posnr numc(6) NOT NULL,
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,posnr,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_swrel_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    submi char(10) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,ekgrp,gjahr,proc_code,proc_counter,step,submi,vbeln) )
;
CREATE TABLE zsd_swrel_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    submi char(10) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,ekgrp,erdat,erzet,gjahr,proc_code,proc_counter,step,submi,vbeln) )
;
CREATE TABLE zsd_swrel_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_swrel_st_t (
    bezei char(50) NOT NULL,
    spras string NOT NULL,
    step string NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_swrel_st_v (
    bezei char(50),
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_swrel_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    ekgrp char(3) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    submi char(10) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (ekgrp,gjahr,proc_code,proc_counter,step,submi,vbeln) )
;
CREATE TABLE zsd_sync1_action (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    ekgrp char(3) NOT NULL,
    endda dats(8),
    event char(32),
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,ekgrp,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_sync1_docs (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_sync1_mess (
    _dataaging dats(8),
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,doc_counter,ekgrp,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_sync1_msg_v (
    add_data char(1300) NOT NULL,
    auart string NOT NULL,
    banfn char(10) NOT NULL,
    doc_counter int4(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8) NOT NULL,
    erdat_va dats(8) NOT NULL,
    ernam char(12) NOT NULL,
    ernam_va char(12) NOT NULL,
    erzet tims(6) NOT NULL,
    erzet_va tims(6) NOT NULL,
    gbstk char(1) NOT NULL,
    global_proc_id char(32) NOT NULL,
    last_step_count int4(10) NOT NULL,
    lock_count numc(10) NOT NULL,
    locked string NOT NULL,
    msgid char(20) NOT NULL,
    msgno numc(3) NOT NULL,
    msgv1 char(50) NOT NULL,
    msgv2 char(50) NOT NULL,
    msgv3 char(50) NOT NULL,
    msgv4 char(50) NOT NULL,
    mtype char(1) NOT NULL,
    noc_code string NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    spart string NOT NULL,
    step char(4) NOT NULL,
    vbeln string NOT NULL,
    vkorg string NOT NULL,
    vtweg string NOT NULL,
    primary key (add_data,auart,banfn,doc_counter,ekgrp,erdat,erdat_va,ernam,ernam_va,erzet,erzet_va,gbstk,global_proc_id,last_step_count,lock_count,locked,msgid,msgno,msgv1,msgv2,msgv3,msgv4,mtype,noc_code,proc_code,proc_counter,spart,step,vbeln,vkorg,vtweg) )
;
CREATE TABLE zsd_sync1_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_sync1_st_t (
    bezei char(50) NOT NULL,
    spras string NOT NULL,
    step string NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_sync1_st_v (
    bezei char(50),
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_sync1_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    banfn char(10) NOT NULL,
    ekgrp char(3) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (banfn,ekgrp,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_sync2_action (
    auart char(4) NOT NULL,
    begda dats(8) NOT NULL,
    ekgrp char(3) NOT NULL,
    endda dats(8),
    event char(32),
    spart char(2) NOT NULL,
    step string NOT NULL,
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,begda,ekgrp,spart,step,vkorg,vtweg) )
;
CREATE TABLE zsd_sync2_docs (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    ebeln char(10) NOT NULL,
    gjahr numc(4) NOT NULL,
    objky char(70),
    objtp char(10),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,ebeln,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_sync2_mess (
    _dataaging dats(8),
    doc_counter int4(10) NOT NULL,
    ebeln char(10) NOT NULL,
    erdat dats(8) NOT NULL,
    ernam char(12),
    erzet tims(6) NOT NULL,
    gjahr numc(4) NOT NULL,
    msgid char(20),
    msgno numc(3),
    msgv1 char(50),
    msgv2 char(50),
    msgv3 char(50),
    msgv4 char(50),
    mtype char(1) NOT NULL,
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (doc_counter,ebeln,erdat,erzet,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_sync2_st (
    step char(4) NOT NULL,
    primary key (step) )
;
CREATE TABLE zsd_sync2_st_t (
    bezei char(50) NOT NULL,
    spras string NOT NULL,
    step string NOT NULL,
    primary key (spras,step) )
;
CREATE TABLE zsd_sync2_steps (
    _dataaging dats(8),
    aedat dats(8),
    aenam char(12),
    aezet tims(6),
    ebeln char(10) NOT NULL,
    erdat dats(8),
    ernam char(12),
    erzet tims(6),
    gjahr numc(4) NOT NULL,
    mtype char(1),
    proc_code char(5) NOT NULL,
    proc_counter char(32) NOT NULL,
    proc_order int4(10),
    step char(4) NOT NULL,
    vbeln char(10) NOT NULL,
    primary key (ebeln,gjahr,proc_code,proc_counter,step,vbeln) )
;
CREATE TABLE zsd_sync_chgtypt (
    change_type string NOT NULL,
    description char(80),
    langu lang(1) NOT NULL,
    primary key (change_type,langu) )
;
CREATE TABLE zsd_sync_contr_v (
    add_attr char(10) NOT NULL,
    change_type string NOT NULL,
    changeable char(1),
    delivery_block char(1),
    description char(80),
    distr_channel char(2) NOT NULL,
    field char(30),
    msg_text char(73),
    msgno string,
    msgty string,
    sales_org char(4) NOT NULL,
    scenario string NOT NULL,
    primary key (add_attr,change_type,distr_channel,sales_org,scenario) )
;
CREATE TABLE zsd_sync_control (
    add_attr char(10) NOT NULL,
    change_type string NOT NULL,
    changeable char(1) NOT NULL,
    delivery_block char(1) NOT NULL,
    description char(80) NOT NULL,
    distr_channel char(2) NOT NULL,
    field char(30) NOT NULL,
    msgno string NOT NULL,
    msgty string NOT NULL,
    po_step char(4) NOT NULL,
    sales_org char(4) NOT NULL,
    scenario string NOT NULL,
    so_step char(4) NOT NULL,
    primary key (add_attr,change_type,distr_channel,sales_org,scenario) )
;
CREATE TABLE zsd_sync_exc_mat (
    material char(18) NOT NULL,
    scenario string,
    primary key (material) )
;
CREATE TABLE zsd_sync_ord_upd (
    distr_channel char(2) NOT NULL,
    item_cat char(4) NOT NULL,
    purchasing_grp char(3) NOT NULL,
    sales_org char(4) NOT NULL,
    scenario string NOT NULL,
    primary key (distr_channel,item_cat,purchasing_grp,sales_org) )
;
CREATE TABLE zsd_sync_so_stat (
    so char(10) NOT NULL,
    status string,
    primary key (so) )
;
CREATE TABLE zsd_tech_calend (
    ident string,
    kunnr string NOT NULL,
    lgort string NOT NULL,
    stat string NOT NULL,
    tjahr numc(4) NOT NULL,
    wefnr string NOT NULL,
    werks string NOT NULL,
    ydays int2(5),
    yearstr lchr(366),
    primary key (kunnr,lgort,stat,tjahr,wefnr,werks) )
;
CREATE TABLE zsd_tech_calend_vb (
    ident string,
    kunnr string,
    lgort string,
    stat string,
    tjahr numc(4),
    updkz char(1),
    wefnr string,
    werks string,
    ydays int2(5),
    yearstr lchr(366))
;
CREATE TABLE zsd_tech_cust (
    btag string,
    die string,
    don char(1),
    fre char(1),
    ident string,
    kunnr string NOT NULL,
    lgort string NOT NULL,
    mit char(1),
    mon char(1),
    name char(30),
    persnr char(8),
    sam char(1),
    son char(1),
    stat string NOT NULL,
    wefnr string NOT NULL,
    werks string NOT NULL,
    primary key (kunnr,lgort,stat,wefnr,werks) )
;
CREATE TABLE zsd_tech_cust_vb (
    btag string,
    die string,
    don char(1),
    fre char(1),
    ident string,
    kunnr string,
    lgort string,
    mit char(1),
    mon char(1),
    name char(30),
    persnr char(8),
    sam char(1),
    son char(1),
    stat string,
    updkz char(1),
    wefnr string,
    werks string)
;
CREATE TABLE zsd_tech_dates (
    month_from string,
    month_name_01 char(3),
    month_name_02 char(3),
    month_name_03 char(3),
    month_to string,
    year_from numc(4),
    year_to numc(4))
;
CREATE TABLE zsd_tech_fields (
    tjahr numc(4),
    tmonth string)
;
CREATE TABLE zsd_tech_wrkkun (
    kunnr string NOT NULL,
    werks string NOT NULL,
    primary key (kunnr,werks) )
;
CREATE TABLE zsd_tech_yearstr (
    day001 numc(2),
    day002 numc(2),
    day003 numc(2),
    day004 numc(2),
    day005 numc(2),
    day006 numc(2),
    day007 numc(2),
    day008 numc(2),
    day009 numc(2),
    day010 numc(2),
    day011 numc(2),
    day012 numc(2),
    day013 numc(2),
    day014 numc(2),
    day015 numc(2),
    day016 numc(2),
    day017 numc(2),
    day018 numc(2),
    day019 numc(2),
    day020 numc(2),
    day021 numc(2),
    day022 numc(2),
    day023 numc(2),
    day024 numc(2),
    day025 numc(2),
    day026 numc(2),
    day027 numc(2),
    day028 numc(2),
    day029 numc(2),
    day030 numc(2),
    day031 numc(2),
    day032 numc(2),
    day033 numc(2),
    day034 numc(2),
    day035 numc(2),
    day036 numc(2),
    day037 numc(2),
    day038 numc(2),
    day039 numc(2),
    day040 numc(2),
    day041 numc(2),
    day042 numc(2),
    day043 numc(2),
    day044 numc(2),
    day045 numc(2),
    day046 numc(2),
    day047 numc(2),
    day048 numc(2),
    day049 numc(2),
    day050 numc(2),
    day051 numc(2),
    day052 numc(2),
    day053 numc(2),
    day054 numc(2),
    day055 numc(2),
    day056 numc(2),
    day057 numc(2),
    day058 numc(2),
    day059 numc(2),
    day060 numc(2),
    day061 numc(2),
    day062 numc(2),
    day063 numc(2),
    day064 numc(2),
    day065 numc(2),
    day066 numc(2),
    day067 numc(2),
    day068 numc(2),
    day069 numc(2),
    day070 numc(2),
    day071 numc(2),
    day072 numc(2),
    day073 numc(2),
    day074 numc(2),
    day075 numc(2),
    day076 numc(2),
    day077 numc(2),
    day078 numc(2),
    day079 numc(2),
    day080 numc(2),
    day081 numc(2),
    day082 numc(2),
    day083 numc(2),
    day084 numc(2),
    day085 numc(2),
    day086 numc(2),
    day087 numc(2),
    day088 numc(2),
    day089 numc(2),
    day090 numc(2),
    day091 numc(2),
    day092 numc(2))
;
CREATE TABLE zsd_text_head_and_line (
    header thead,
    lines table_of_tline)
;
CREATE TABLE zsd_tmon_logging (
    identifier char(20) NOT NULL,
    idocnum numc(16) NOT NULL,
    timestamp dec(21) NOT NULL,
    trref_l char(10) NOT NULL,
    username char(12) NOT NULL,
    vbeln_vl char(10) NOT NULL,
    primary key (identifier,idocnum,timestamp,trref_l,username,vbeln_vl) )
;
CREATE TABLE zsd_track_tmstat (
    def char(50),
    ztmstat char(1) NOT NULL,
    primary key (ztmstat) )
;
CREATE TABLE zsd_trackin_mail (
    deliveryservice char(10) NOT NULL,
    mail char(241) NOT NULL,
    primary key (deliveryservice,mail) )
;
CREATE TABLE zsd_tracking (
    headerflag string,
    icons char(132),
    lfdat dats(8) NOT NULL,
    posnr numc(6) NOT NULL,
    prdcd char(10),
    routecode char(12),
    routeldat dats(8),
    trackn char(35) NOT NULL,
    tracktext char(80),
    tracktstmp dec(15),
    trackurl char(256),
    trackz tims(6) NOT NULL,
    trkstatd char(4),
    vbeln char(10) NOT NULL,
    vbtyp string NOT NULL,
    xsisrvc char(10),
    xsitd char(10) NOT NULL,
    ztmstat char(1),
    primary key (lfdat,posnr,trackn,trackz,vbeln,vbtyp,xsitd) )
;
CREATE TABLE zsd_tracking_alv_rep (
    erdat dats(8),
    ernam char(12),
    headerflag string,
    icons char(132),
    kunag char(10),
    kunnr char(10),
    lfdat dats(8),
    matkl char(9),
    matnr char(18),
    posnr numc(6),
    prdcd char(10),
    routecode char(12),
    routeldat dats(8),
    trackn char(35),
    tracktext char(80),
    tracktstmp dec(15),
    trackurl char(256),
    trackz tims(6),
    trkstatd char(4),
    vbeln char(10),
    vbtyp string,
    vkorg char(4),
    vstel char(4),
    xsisrvc char(10),
    xsitd char(10),
    ztmstat char(1))
;
CREATE TABLE zsd_tracking_ico (
    iconname char(30),
    kepid char(10) NOT NULL,
    rmcode char(3),
    trkstatd char(4) NOT NULL,
    primary key (kepid,trkstatd) )
;
CREATE TABLE zsd_tracking_lan (
    kepid string NOT NULL,
    shorttext char(30),
    spras lang(1) NOT NULL,
    trkstatd string NOT NULL,
    primary key (kepid,spras,trkstatd) )
;
CREATE TABLE zsd_tracking_out (
    forward_name char(30),
    forward_partner char(10),
    headerflag string,
    icons char(132),
    lfdat dats(8),
    posnr numc(6),
    prdcd char(10),
    routecode char(12),
    routeldat dats(8),
    trackn char(35),
    tracktext char(80),
    tracktstmp dec(15),
    trackurl char(256),
    trackz tims(6),
    trkstatd char(4),
    vbeln char(10),
    vbtyp string,
    xsisrvc char(10),
    xsitd char(10),
    ztmstat char(1))
;
CREATE TABLE zsd_v1_mailsend (
    auart string NOT NULL,
    kschl char(4) NOT NULL,
    mailsender char(12),
    title_p_number string,
    vkorg string NOT NULL,
    primary key (auart,kschl,vkorg) )
;
CREATE TABLE zsd_v2_mailsend (
    kschl string NOT NULL,
    lfart string NOT NULL,
    mailsender char(12),
    title_p_number string,
    vkorg string NOT NULL,
    primary key (kschl,lfart,vkorg) )
;
CREATE TABLE zsd_v_block_area (
    area char(3) NOT NULL,
    descr char(40),
    spras lang(1) NOT NULL,
    primary key (area,spras) )
;
CREATE TABLE zsd_v_block_mail (
    area string NOT NULL,
    block string NOT NULL,
    email char(241) NOT NULL,
    field string NOT NULL,
    primary key (area,block,email,field) )
;
CREATE TABLE zsd_v_tech_main0 (
    btag string,
    die string,
    don char(1),
    fre char(1),
    ident string,
    kunnr string NOT NULL,
    lgort string NOT NULL,
    mit char(1),
    mon char(1),
    name char(30),
    persnr char(8),
    sam char(1),
    son char(1),
    stat string NOT NULL,
    wefnr string NOT NULL,
    werks string NOT NULL,
    primary key (kunnr,lgort,stat,wefnr,werks) )
;
CREATE TABLE zsd_v_tech_main1 (
    btag string,
    die string,
    don char(1),
    fre char(1),
    ident string,
    kunnr string NOT NULL,
    lgort string NOT NULL,
    mit char(1),
    mon char(1),
    name char(30),
    persnr char(8),
    sam char(1),
    son char(1),
    stat string NOT NULL,
    tjahr numc(4) NOT NULL,
    wefnr string NOT NULL,
    werks string NOT NULL,
    ydays int2(5),
    yearstr lchr(366),
    primary key (kunnr,lgort,stat,tjahr,wefnr,werks) )
;
CREATE TABLE zsd_va_equi_s (
    equnr char(18),
    obknr dec(19),
    obzae int4(10),
    posnr_next numc(6),
    posnr_va_beu numc(6),
    sernr char(18),
    vbeln_next char(10),
    vbeln_va_beu char(10),
    vbeln_va_noc char(35),
    vbtyp_next char(4))
;
CREATE TABLE zsd_vbak_vbeln (
    vbeln char(10))
;
CREATE TABLE zsd_vbap_append (
    zzarktx char(1),
    zzcalctype string,
    zzequnr char(18),
    zzingrp char(3),
    zziwerk char(4),
    zzkzdiv char(60),
    zzmulti numc(8),
    zzposnr numc(6),
    zzsworder char(50),
    zzvart char(3))
;
CREATE TABLE zsd_vbap_appendx (
    zposnr char(1),
    zzarktx char(1),
    zzcalctype char(1),
    zzequnr char(1),
    zzingrp char(1),
    zziwerk char(1),
    zzkzdiv char(1),
    zzmulti char(1),
    zzsworder char(1),
    zzvart char(1))
;
CREATE TABLE zsd_vbeln_banfn (
    banfn char(10) NOT NULL,
    ekgrp string NOT NULL,
    posnr string NOT NULL,
    vbeln string NOT NULL,
    primary key (banfn,ekgrp,posnr,vbeln) )
;
CREATE TABLE zsd_vbeln_data (
    customer char(10),
    delivery char(10),
    equipment char(18),
    sales_order char(10),
    submission char(10))
;
CREATE TABLE zsd_vbeln_ebeln (
    ebeln string NOT NULL,
    vbeln string NOT NULL,
    primary key (ebeln,vbeln) )
;
CREATE TABLE zsd_vbeln_vtnum (
    augru char(3),
    codegruppe char(8),
    kbetr dec(11),
    konm_posnr numc(6),
    konm_vbeln char(10),
    preisgruppe numc(2),
    status char(1),
    vbeln char(10),
    vtnum char(10),
    vtpos numc(6),
    waers cuky(5))
;
CREATE TABLE zsd_vbfa_ecc (
    abges fltp(16),
    aedat dats(8),
    bdart char(2),
    brgew quan(15),
    bwart char(3),
    cmeth string,
    erdat dats(8),
    erzet tims(6),
    fktyp string,
    fplnr char(10),
    fpltr numc(6),
    gewei unit(3),
    kzbef char(1),
    lgnum char(3),
    logsys char(10),
    matnr char(40),
    meins unit(3),
    mjahr numc(4),
    ntgew quan(13),
    plart char(1),
    plmin string,
    posnn numc(6),
    posnv numc(6),
    rfmng quan(15),
    rfmng_flo fltp(16),
    rfmng_flt fltp(16),
    rfwrt curr(15),
    sobkz char(1),
    sonum char(16),
    stufe numc(2),
    taqui char(1),
    vbeln char(10),
    vbelv char(10),
    vbtyp_n char(1),
    vbtyp_v char(1),
    voleh unit(3),
    volum quan(15),
    vrkme unit(3),
    waers cuky(5),
    wbsta char(1))
;
CREATE TABLE zsd_vbkd_leasd_a_oz (
    zz_lease_desk string)
;
CREATE TABLE zsd_vbkd_leasd_a_ozx (
    zz_lease_desk char(1))
;
CREATE TABLE zsd_vbkd_leasd_ax (
    zz_lease_desk char(1))
;
CREATE TABLE zsd_vbkd_ops (
    zz_ops_detail char(1))
;
CREATE TABLE zsd_vbkd_ops_a (
    zz_ops_detail char(1))
;
CREATE TABLE zsd_vbkd_ops_a_oz (
    zz_ops_detail char(1))
;
CREATE TABLE zsd_vbkd_ops_a_ozx (
    zz_ops_detail char(1))
;
CREATE TABLE zsd_vbkd_ops_ax (
    zz_ops_detail char(1))
;
CREATE TABLE zsd_vbkd_ops_x (
    zz_ops_detail char(1))
;
CREATE TABLE zsd_vbkd_vbap_leasd_a_oz (
    zz_enachw char(1),
    zz_enachw_p char(1),
    zz_gutsch char(1),
    zz_lease_desk string,
    zz_nopra char(1),
    zz_nulrg char(1),
    zz_onere char(1),
    zz_sumrg char(1),
    zz_zterm char(4))
;
CREATE TABLE zsd_vbkd_vbap_leasd_a_ozx (
    zz_enachw char(1),
    zz_enachw_p char(1),
    zz_gutsch char(1),
    zz_lease_desk char(1),
    zz_nopra char(1),
    zz_nulrg char(1),
    zz_onere char(1),
    zz_sumrg char(1),
    zz_zterm char(1))
;
CREATE TABLE zsd_vbkdops_a (
    zz_ops_detail char(1))
;
CREATE TABLE zsd_vf04_user (
    zuser char(12) NOT NULL,
    primary key (zuser) )
;
CREATE TABLE zsd_vscode_pt (
    plz char(10) NOT NULL,
    spediteur char(10) NOT NULL,
    vsbed string NOT NULL,
    zvsb_code_pt char(10),
    primary key (plz,spediteur,vsbed) )
;
CREATE TABLE zsd_wph (
    equnr char(18),
    erdat dats(8),
    erzet tims(6),
    event_dat dats(8),
    event_zet tims(6),
    license_created_dat dats(8) NOT NULL,
    license_created_zet tims(6) NOT NULL,
    license_expires_dat dats(8),
    license_expires_zet tims(6),
    license_name char(100),
    license_type char(20),
    maktx char(40),
    matnr char(18),
    posnr numc(6) NOT NULL,
    seq_no int4(10) NOT NULL,
    sernr char(18),
    ship_to char(10),
    sold_to char(10),
    value_current int4(10),
    value_max int4(10),
    value_previous int4(10),
    vbeln char(10) NOT NULL,
    vbeln_wph char(10),
    primary key (license_created_dat,license_created_zet,posnr,seq_no,vbeln) )
;
CREATE TABLE zsd_xml_format (
    descr char(60) NOT NULL,
    xml_f int1(3) NOT NULL,
    primary key (descr,xml_f) )
;
CREATE TABLE zsd_zdwp_csv_str (
    field char(60),
    posnr int1(3) NOT NULL,
    primary key (posnr) )
;
CREATE TABLE zsd_zdwp_nast (
    erdat dats(8) NOT NULL,
    eruhr tims(6) NOT NULL,
    kappl char(2) NOT NULL,
    kschl char(4) NOT NULL,
    objky char(30) NOT NULL,
    parnr char(10) NOT NULL,
    parvw char(2) NOT NULL,
    spras lang(1) NOT NULL,
    vstat string,
    primary key (erdat,eruhr,kappl,kschl,objky,parnr,parvw,spras) )
;
CREATE TABLE zsd_zkli_upd (
    abgru string,
    abgru_txt char(40),
    arktx char(40),
    compl_dlv char(132),
    kwmeng quan(15),
    matnr char(18),
    posnr numc(6),
    pstyv char(4),
    spart char(2),
    uepos numc(6),
    vbeln char(10),
    vkorg char(4),
    vrkme unit(3),
    vtweg char(2),
    werks char(4))
;
CREATE TABLE zsd_zora (
    aktiv char(1),
    auart char(4) NOT NULL,
    fkart char(4) NOT NULL,
    vkorg char(4) NOT NULL,
    primary key (auart,fkart,vkorg) )
;
CREATE TABLE zsd_zora_text (
    auart char(4) NOT NULL,
    check_table char(16),
    check_table2 char(16),
    field char(30),
    field2 char(30),
    kunnr char(10) NOT NULL,
    std_txt char(70),
    textid char(4),
    value char(30),
    value2 char(30),
    vkorg char(4) NOT NULL,
    vtweg char(2) NOT NULL,
    primary key (auart,kunnr,vkorg,vtweg) )
;

