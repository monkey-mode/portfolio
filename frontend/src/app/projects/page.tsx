import Image from 'next/image';

async function getProjects() {
  const res = await fetch('http://localhost:8080/api/projects', { cache: 'no-store' });
  if (!res.ok) return [];
  return res.json();
}

export default async function Projects() {
  const projects = await getProjects();

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <h1 className="text-3xl font-bold mb-8 text-gray-900 dark:text-white">Featured Projects</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        {projects.map((project: any) => (
          <div key={project.id} className="group relative rounded-2xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 overflow-hidden hover:shadow-xl transition-all duration-300 flex flex-col h-full">
            <div className="relative h-48 w-full bg-gray-200 dark:bg-gray-800">
               {project.image_url && (
                   <Image 
                     src={project.image_url} 
                     alt={project.title} 
                     fill 
                     className="object-cover group-hover:scale-105 transition-transform duration-500" 
                   />
               )}
            </div>
            <div className="p-6 flex-1 flex flex-col">
              <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-2">{project.title}</h3>
              <p className="text-gray-600 dark:text-gray-400 mb-4 flex-1">{project.description}</p>
              <div className="flex flex-wrap gap-2 mb-4">
                {project.technologies.map((tech: string) => (
                  <span key={tech} className="px-2 py-1 text-xs font-medium rounded-md bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-300">
                    {tech}
                  </span>
                ))}
              </div>
              <div className="flex space-x-3 mt-auto">
                {project.repo_url && (
                    <a href={project.repo_url} target="_blank" rel="noopener noreferrer" className="text-sm font-medium text-gray-900 dark:text-white hover:underline">
                        View Code
                    </a>
                )}
                 {project.demo_url && (
                    <a href={project.demo_url} target="_blank" rel="noopener noreferrer" className="text-sm font-medium text-gray-900 dark:text-white hover:underline">
                        Live Demo
                    </a>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>
      {projects.length === 0 && (
          <p className="text-center text-gray-500 mt-10">No projects found (or backend is unreachable).</p>
      )}
    </div>
  );
}
