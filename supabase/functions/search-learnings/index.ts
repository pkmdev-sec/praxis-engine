// Supabase Edge Function: search-learnings
// Full-text search over learnings (no OpenAI required)

import { serve } from "https://deno.land/std@0.168.0/http/server.ts";
import { createClient } from "https://esm.sh/@supabase/supabase-js@2";

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Headers": "authorization, x-client-info, apikey, content-type",
};

interface SearchInput {
  query: string;
  domains?: string[];
  limit?: number;
}

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

    const input: SearchInput = await req.json();

    if (!input.query) {
      throw new Error("Missing required field: query");
    }

    const limit = input.limit ?? 5;

    const supabase = createClient(supabaseUrl, supabaseAnonKey);

    // Use full-text search function
    const { data, error } = await supabase.rpc("search_learnings_text", {
      query_text: input.query,
      match_count: limit,
      filter_domains: input.domains || null,
    });

    if (error) {
      throw new Error(`Supabase error: ${error.message}`);
    }

    return new Response(
      JSON.stringify({
        success: true,
        query: input.query,
        count: data?.length || 0,
        learnings: (data || []).map((l: any) => ({
          slug: l.slug,
          name: l.name,
          content: l.content,
          domains: l.domains,
          rank: l.rank,
        })),
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
