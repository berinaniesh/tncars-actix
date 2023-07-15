DROP TABLE IF EXISTS public.posts;

CREATE TABLE IF NOT EXISTS public.posts
(
    id integer NOT NULL DEFAULT nextval('posts_id_seq'::regclass),
    title character varying COLLATE pg_catalog."default" NOT NULL,
    user_id integer NOT NULL DEFAULT nextval('posts_user_id_seq'::regclass),
    price integer NOT NULL,
    model_year integer NOT NULL,
    km_driven integer NOT NULL,
    description text COLLATE pg_catalog."default" NOT NULL,
    location character varying COLLATE pg_catalog."default" NOT NULL,
    is_sold boolean NOT NULL DEFAULT false,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT posts_pkey PRIMARY KEY (id),
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

-- ALTER TABLE IF EXISTS public.posts
--    OWNER to pg_database_owner;

-- COMMENT ON CONSTRAINT user_id ON public.posts
--    IS 'link posts to user_id';
