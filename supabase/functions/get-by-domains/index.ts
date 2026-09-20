// Supabase Edge Function: get-by-domains
// Retrieve all learnings matching specified domains

import { serve } from "https://deno.land/std@0.168.0/http/server.ts";
import { createClient } from "https://esm.sh/@supabase/supabase-js@2";

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Headers": "authorization, x-client-info, apikey, content-type",
};

serve(async (req) => {
  // Handle CORS preflight
  if (req.method === "OPTIONS") {
    return new Response("ok", { headers: corsHeaders });
  }

  try {
    const supabaseUrl = Deno.env.get("SUPABASE_URL");
    const supabaseAnonKey = Deno.env.get("SUPABASE_ANON_KEY");

    if (!supabaseUrl || !supabaseAnonKey) {
      throw new Error("Missing required environment variables");
    }

    const { domains } = await req.json();

    if (!domains || !Array.isArray(domains) || domains.length === 0) {
      throw new Error("Missing required field: domains (array of strings)");
    }

    const supabase = createClient(supabaseUrl, supabaseAnonKey);

    const { data, error } = await supabase.rpc("get_learnings_by_domains", {
      target_domains: domains,
    });

    if (error) {
      throw new Error(`Supabase error: ${error.message}`);
    }

    return new Response(
      JSON.stringify({
        success: true,
        domains: domains,
        count: data.length,
        learnings: data,
      }),
      {
        headers: { ...corsHeaders, "Content-Type": "application/json" },
        status: 200,
      }
    );
  } catch (error) {
    return new Response(
      JSON.stringify({
        success: false,
        error: error.message,
      }),
      {
        headers: { ...corsHeaders, "Content-Type": "application/json" },
        status: 400,
      }
    );
  }
});
