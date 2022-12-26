import { serve } from "https://deno.land/std@0.170.0/http/mod.ts";
import { serveDir } from "https://deno.land/std@0.170.0/http/file_server.ts";

const config = {
  fsRoot : "../www"
};
serve((req) => serveDir(req, config));