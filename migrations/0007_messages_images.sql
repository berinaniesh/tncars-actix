-- DROP TABLE IF EXISTS public.messages_images;

CREATE SEQUENCE messages_images_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


CREATE TABLE IF NOT EXISTS public.messages_images
(
    id integer NOT NULL DEFAULT nextval('messages_images_id_seq'::regclass),
    user_id integer NOT NULL,
    image_link character varying COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT messages_images_pkey PRIMARY KEY (id),
    CONSTRAINT user_link FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.messages_images
    OWNER to pg_database_owner;

CREATE TRIGGER update_customer_modtime BEFORE UPDATE ON messages_images FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();