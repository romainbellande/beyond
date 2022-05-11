function Page404() {
  return (
    <div className="mt-8 flex flex-col items-center">
      <h1 className="text-6xl font-semibold text-gray-700 dark:text-gray-200">
        404
      </h1>
      <p className="text-gray-700 dark:text-gray-300">
        Page not found. Check the address or{' '}
        <a
          className="text-purple-600 hover:underline dark:text-purple-300"
          href="/"
        >
          go back
        </a>
        .
      </p>
    </div>
  );
}

export default Page404;
