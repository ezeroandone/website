const SESSION = process.env.SESSION_COOKIE;
const BASE_URL = process.env.BASE_URL ?? 'https://ezeroandone.io';
const res = await fetch(`${BASE_URL}/api/admin/clients`, { headers: { Cookie: `session=${SESSION}` } });
const clients = await res.json();
clients.forEach(c => console.log(c.id, `"${c.name}"`, c.logo_url));
