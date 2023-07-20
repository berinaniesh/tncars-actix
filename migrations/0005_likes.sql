-- DROP TABLE IF EXISTS public.likes;

CREATE SEQUENCE likes_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


CREATE TABLE IF NOT EXISTS public.likes
(
    id integer NOT NULL DEFAULT nextval('likes_id_seq'::regclass),
    user_id integer NOT NULL,
    post_id integer NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
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

ALTER TABLE IF EXISTS public.likes
    OWNER to pg_database_owner;

CREATE TRIGGER update_customer_modtime BEFORE UPDATE ON likes FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();