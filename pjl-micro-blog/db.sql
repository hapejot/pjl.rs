-- Table: public.posts

DROP TABLE IF EXISTS public.posts;

CREATE TABLE IF NOT EXISTS public.posts
(
    id character varying(50) COLLATE pg_catalog."default",
    title character varying(100) COLLATE pg_catalog."default",
    content character varying(4000) COLLATE pg_catalog."default",
	primary key (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.posts
    OWNER to peter;