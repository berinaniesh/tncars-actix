DROP TABLE IF EXISTS public.posts_images;

CREATE SEQUENCE posts_images_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


CREATE TABLE IF NOT EXISTS public.posts_images
(
    id integer NOT NULL DEFAULT nextval('posts_images_id_seq'::regclass),
    post_id integer NOT NULL,
    image_url character varying COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT posts_images_pkey PRIMARY KEY (id),
    CONSTRAINT post_key FOREIGN KEY (post_id)
        REFERENCES public.posts (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

-- ALTER TABLE IF EXISTS public.posts_images
--    OWNER to pg_database_owner;

-- COMMENT ON CONSTRAINT post_key ON public.posts_images
--    IS 'link image to post';
