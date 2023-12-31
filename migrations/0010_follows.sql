-- Table: public.follows

DROP TABLE IF EXISTS public.follows;

CREATE SEQUENCE follows_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE IF NOT EXISTS public.follows
(
    id integer NOT NULL DEFAULT nextval('follows_id_seq'::regclass),
    from_user integer NOT NULL,
    to_user integer NOT NULL,
    UNIQUE (from_user, to_user),
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT follows_pkey PRIMARY KEY (id),
    CONSTRAINT from_user_link FOREIGN KEY (from_user)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT to_user_link FOREIGN KEY (to_user)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.follows
    OWNER to pg_database_owner;