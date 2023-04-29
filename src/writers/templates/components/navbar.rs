use crate::writers::write_to_file;
use crate::Project;
use std::io::Error;

pub fn write_to_navbar(project: &Project) -> Result<(), Error> {
    let contents = r#"<header class='relative'>
    <div class='bg-gray-900 pt-6'>
        <nav class='relative mx-auto flex max-w-7xl items-center justify-between px-6' aria-label='Global'>
            <div class='flex flex-1 items-center'>
                <div class='flex w-full items-center justify-between md:w-auto'>
                    <a href='/'>
                        <span class='sr-only'>Your Company</span>
                        <img class='h-8 w-auto sm:h-10'
                            src='https://tailwindui.com/img/logos/mark.svg?from-color=teal&from-shade=200&to-color=cyan&to-shade=400&toShade=400'
                            alt=''>
                    </a>
                    <div class='-mr-2 flex items-center md:hidden'>
                        <button type='button'
                            class='focus-ring-inset inline-flex items-center justify-center rounded-md bg-gray-900 p-2 text-gray-400 hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-white'
                            aria-expanded='false'>
                            <span class='sr-only'>Open main menu</span>
                            <!-- Heroicon name: outline/bars-3 -->
                            <svg class='h-6 w-6' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                                stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                                <path stroke-linecap='round' stroke-linejoin='round'
                                    d='M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5' />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
            <div class='hidden md:flex md:items-center md:space-x-6'>
                <a href='/login' class='text-base font-medium text-white hover:text-gray-300'>Log in</a>
            </div>
        </nav>
    </div>

    <!--
        Mobile menu, show/hide based on menu open state.

        Entering: 'duration-150 ease-out'
          From: 'opacity-0 scale-95'
          To: 'opacity-100 scale-100'
        Leaving: 'duration-100 ease-in'
          From: 'opacity-100 scale-100'
          To: 'opacity-0 scale-95'
      -->
    <div class='absolute inset-x-0 top-0 origin-top transform p-2 transition md:hidden'>
        <div class='overflow-hidden rounded-lg bg-white shadow-md ring-1 ring-black ring-opacity-5'>
            <div class='flex items-center justify-between px-5 pt-4'>
                <div>
                    <img class='h-8 w-auto'
                        src='https://tailwindui.com/img/logos/mark.svg?from-color=teal&from-shade=500&to-color=cyan&to-shade=600&toShade=600'
                        alt=''>
                </div>
                <div class='-mr-2'>
                    <button type='button'
                        class='inline-flex items-center justify-center rounded-md bg-white p-2 text-gray-400 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-cyan-600'>
                        <span class='sr-only'>Close menu</span>
                        <!-- Heroicon name: outline/x-mark -->
                        <svg class='h-6 w-6' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                            stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                            <path stroke-linecap='round' stroke-linejoin='round' d='M6 18L18 6M6 6l12 12' />
                        </svg>
                    </button>
                </div>
            </div>
            <div class='pt-5 pb-6'>
                <div class='mt-6 px-5'>
                    <p class='text-center text-base font-medium text-gray-500'>Ready to start building? <a href='#'
                            class='text-gray-900 hover:underline'>Login</a></p>
                </div>
            </div>
        </div>
    </div>
</header>"#;
    write_to_file(&project.navbar_component, contents.as_bytes())
}
