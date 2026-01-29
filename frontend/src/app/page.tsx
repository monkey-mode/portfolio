async function getProfile() {
  const res = await fetch('http://localhost:8080/api/profile', { cache: 'no-store' });
  if (!res.ok) {
    // Fallback for demo if backend not running
    return {
        name: "Demo User",
        title: "Software Engineer",
        summary: "Backend unavailable. Please ensure Rust server is running.",
        email: "demo@example.com",
        github_url: "#",
        linkedin_url: "#"
    };
  }
  return res.json();
}

async function getSkills() {
  const res = await fetch('http://localhost:8080/api/skills', { cache: 'no-store' });
  if (!res.ok) return [];
  return res.json();
}

export default async function Home() {
  const profile = await getProfile();
  const skills = await getSkills();

  return (
    <div className="flex flex-col items-center justify-center min-h-[calc(100vh-4rem)] text-center px-4">
      <section className="space-y-6 max-w-3xl">
        <h1 className="text-4xl md:text-6xl font-bold tracking-tighter bg-clip-text text-transparent bg-gradient-to-r from-blue-600 to-purple-600 dark:from-blue-400 dark:to-purple-400">
          Hi, I'm {profile.name}
        </h1>
        <p className="text-xl md:text-2xl text-gray-600 dark:text-gray-300 font-light">
          {profile.title}
        </p>
        <p className="text-lg text-gray-500 dark:text-gray-400 max-w-2xl mx-auto leading-relaxed">
          {profile.summary}
        </p>
        
        <div className="flex justify-center space-x-4 pt-4">
          {profile.github_url && (
            <a href={profile.github_url} target="_blank" rel="noopener noreferrer" className="px-6 py-3 rounded-full bg-gray-900 dark:bg-white text-white dark:text-black font-semibold hover:opacity-90 transition-opacity">
              GitHub
            </a>
          )}
          {profile.linkedin_url && (
             <a href={profile.linkedin_url} target="_blank" rel="noopener noreferrer" className="px-6 py-3 rounded-full bg-blue-600 text-white font-semibold hover:bg-blue-700 transition-colors">
              LinkedIn
            </a>
          )}
        </div>
      </section>

      {skills.length > 0 && (
        <section className="mt-20 w-full max-w-4xl">
           <h2 className="text-2xl font-bold mb-8 text-gray-800 dark:text-gray-200">Skills</h2>
           <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
              {skills.map((skill: any) => (
                  <div key={skill.id} className="p-4 rounded-xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900/50 hover:border-blue-500 transition-colors">
                      <div className="font-medium text-gray-900 dark:text-gray-100">{skill.name}</div>
                      <div className="text-sm text-gray-500">{skill.category}</div>
                  </div>
              ))}
           </div>
        </section>
      )}
    </div>
  );
}
