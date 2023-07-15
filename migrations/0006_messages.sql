DROP TABLE IF EXISTS public.messages;

CREATE TABLE IF NOT EXISTS public.messages
(
    id bigint NOT NULL DEFAULT nextval('messages_id_seq'::regclass),
    from_user integer NOT NULL DEFAULT nextval('messages_from_user_seq'::regclass),
    to_user integer NOT NULL DEFAULT nextval('messages_to_user_seq'::regclass),
    message character varying COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT messages_pkey PRIMARY KEY (id),
    CONSTRAINT from_link FOREIGN KEY (from_user)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT to_link FOREIGN KEY (to_user)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
)

TABLESPACE pg_default;

-- ALTER TABLE IF EXISTS public.messages
--    OWNER to pg_database_owner;
