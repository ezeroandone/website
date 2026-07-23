import type { PageServerLoad } from './$types';
import { apiFetch } from '$lib/api';

interface WorkPost {
  id: string;
  type: 'work';
  slug: string;
  title: string;
  summary: string;
  body_md: string;
  author_id: string | null;
  published_at: number | null;
  updated_at: number;
  published: boolean;
  featured_image_url: string;
  category: string;
  tags: string;
  project_type: string;
  technologies: string;
  material_icon: string;
  // Author joined fields
  username?: string | null;
  name?: string | null;
  job_title?: string | null;
  avatar_url?: string | null;
}

interface WorkTeamMember {
  id: string;
  post_id: string;
  staff_id: string | null;
  staff_name: string | null;
  staff_username: string | null;
  staff_avatar_url: string | null;
  staff_job_title: string | null;
  ext_name: string;
  ext_role: string;
  ext_url: string;
  sort_order: number;
}

export const load: PageServerLoad = async ({ fetch, params }) => {
  const [post, team] = await Promise.all([
    apiFetch<WorkPost>(fetch, `/api/work/${params.slug}`),
    apiFetch<WorkTeamMember[]>(fetch, `/api/work/${params.slug}/team`).catch(() => [] as WorkTeamMember[]),
  ]);
  return { post, team };
};
