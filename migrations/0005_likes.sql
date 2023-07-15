DROP TABLE IF EXISTS public.likes;

CREATE TABLE IF NOT EXISTS public.likes
(
    id integer NOT NULL DEFAULT nextval('likes_id_seq'::regclass),
    user_id integer NOT NULL DEFAULT nextval('likes_user_id_seq'::regclass),
    post_id integer NOT NULL DEFAULT nextval('likes_post_id_seq'::regclass),
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT likes_pkey PRIMARY KEY (id),
    CONSTRAINT post_link FOREIGN KEY (post_id)
        REFERENCES public.posts (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT user_link FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
)

TABLESPACE pg_default;

-- ALTER TABLE IF EXISTS public.likes
--    OWNER to pg_database_owner;
