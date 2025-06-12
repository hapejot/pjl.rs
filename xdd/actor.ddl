CREATE TABLE actor (
    id string,
    name string,
    bio string,
    birthPlace string,
    birthday string,
    gender string,
    height string,
    lastSceneReleaseDate string,
    measurements string,
    rank string,
    weight string,
    primary key (id) );
CREATE TABLE file (
    id string,
    pathid string,
    size int,
    name string,
    primary key (id) );
CREATE TABLE tag (
    id string,
    name string,
    category string,
    categoryOrder string,
    primary key (id) );
CREATE TABLE actor_tag (
    actorid string,
    tagid string,
    primary key (actorid,tagid) );
CREATE TABLE scene (
    id string,
    title string,
    description string, 
    actors, 
    tags, 
    video_length, 
    video_1080p_sizeBytes, 
    video_1080p_download, 
    video_1080p_view, 
    video_1080p_format,
    primary key (id) );
CREATE TABLE scene_tag (
    sceneid string,
    tagid string,
    primary key (sceneid,tagid) );
CREATE TABLE actor_scene (
    actorid string,
    sceneid string,
    primary key (actorid,sceneid) );
CREATE TABLE video (
    id string,
    path string,
    primary key (id) );
CREATE TABLE scene_video (
    sceneid string,
    videoid string,
    primary key (sceneid,videoid) );
CREATE TABLE path (
    id string,
    name string,
    primary key (id) );
CREATE TABLE path_scene (
    pathid string,
    sceneid string,
    ignore boolean,
    primary key (pathid) );



CREATE VIEW tag_stats as select tagid, count(*) cnt from scene_tag group by tagid
/* tag_stats(tagid,cnt) */;
CREATE VIEW d_scene_tag as select scene.id, scene.title, tag.name from scene join scene_tag on scene.id = scene_tag.sceneid join tag on tag.id = scene_tag.tagid
/* d_scene_tag(id,title,name) */;
CREATE VIEW d_file_scene as select path.id as pathid, file.id as fileid, path.name as path, file.name as filename,  scene.id, scene.title from path_scene  join scene on path_scene.sceneid = scene.id join path on path.id = path_scene.pathid join file on file.pathid = path.id
/* d_file_scene(pathid,fileid,path,filename,id,title) */;
CREATE VIEW d_file as select path.id as pathid, file.id as fileid, path.name as path, file.name as name from path, file where path.id = file.pathid
/* d_file(pathid,fileid,path,name) */;
CREATE VIEW d_scene_actor as 
    select distinct scene.id, 
                    scene.title, 
                    actor.id    as actorid,
                    actor.name,
                    case when path_scene.pathid is null then 0 else 1 end as has_files
    from scene 
    join actor_scene on scene.id = actor_scene.sceneid 
    join actor on actor.id = actor_scene.actorid
    left outer join path_scene on scene.id = path_scene.sceneid
/* d_scene_actor(id,title,actorid,name,has_files) */;
