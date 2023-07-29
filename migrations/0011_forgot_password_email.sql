-- Table: public.forgot_password_email

-- DROP TABLE IF EXISTS public.forgot_password_email;

CREATE SEQUENCE forgot_password_email_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE IF NOT EXISTS public.forgot_password_email
(
    id integer NOT NULL DEFAULT nextval('forgot_password_email_id_seq'::regclass),
    user_id integer NOT NULL,
    otp character varying COLLATE pg_catalog."default" NOT NULL,
    verify_url character varying COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at timestamp with time zone NOT NULL,
    CONSTRAINT forgot_password_email_pkey PRIMARY KEY (id),
    CONSTRAINT forgot_password_email_verify_url_key UNIQUE (verify_url),
    CONSTRAINT forgot_password_email_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.forgot_password_email
    OWNER to pg_database_owner;