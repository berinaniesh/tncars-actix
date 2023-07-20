-- Table: public.delete_users

-- DROP TABLE IF EXISTS public.delete_users;

CREATE SEQUENCE delete_users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE IF NOT EXISTS public.delete_users
(
    id integer NOT NULL DEFAULT nextval('delete_users_id_seq'::regclass),
    user_id integer NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT delete_users_pkey PRIMARY KEY (id),
    CONSTRAINT user_unique UNIQUE (user_id)
        INCLUDE(user_id),
    CONSTRAINT user_link FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.delete_users
    OWNER to pg_database_owner;