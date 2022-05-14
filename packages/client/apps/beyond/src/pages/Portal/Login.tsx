import { FC, useEffect } from 'react';
import { Button } from '@client/ui';
import { LockClosedIcon } from '@heroicons/react/solid';
import { Formik, Form, Field } from 'formik';
import { Credentials } from '@client/beyond/interfaces';
import { useLoginMutation } from '@client/beyond/store/api';
import { useNavigate } from 'react-router-dom';
import { selectJwt, setJwt } from '@client/beyond/store/player';
import { useAppSelector } from '@client/beyond/store';
import { useDispatch } from 'react-redux';

const Login: FC = () => {
  const [login, result] = useLoginMutation();
  const navigate = useNavigate();
  const jwt = useAppSelector(selectJwt);
  const dispatch = useDispatch();

  const initialValues: Credentials = {
    username: '',
    password: '',
  };

  useEffect(() => {
    if (result.data) {
      const { jwt } = result.data;
      dispatch(setJwt(jwt));
      navigate('/');
    }
  }, [result]);

  const onSubmit = async (credentials: Credentials) => {
    await login(credentials);
  };

  return (
    <div className="bg-slate-900 p-4 rounded-md shadow-md">
      <div>
        <img
          className="mx-auto h-12 w-aut"
          src="https://tailwindui.com/img/logos/workflow-mark-indigo-600.svg"
          alt="Workflow"
        />
        <h2 className="mt-6 text-center text-3xl font-extrabold">
          Sign in to your account
        </h2>
        <p className="mt-2 text-center text-sm text-gray-600">
          Or
          <a
            href="#"
            className="font-medium text-indigo-600 hover:text-indigo-500"
          >
            {' '}
            create an account{' '}
          </a>
        </p>
      </div>
      <Formik initialValues={initialValues} onSubmit={onSubmit}>
        <Form className="mt-8 space-y-6" action="#" method="POST">
          <input type="hidden" name="remember" value="true" />
          <div className="rounded-md shadow-sm space-y-4">
            <div>
              <label htmlFor="username" className="sr-only">
                Email address
              </label>
              <Field
                id="username"
                name="username"
                className="field"
                type="text"
                required
                placeholder="Username"
              />
            </div>
            <div>
              <label htmlFor="password" className="sr-only">
                Password
              </label>
              <Field
                id="password"
                name="password"
                className="field"
                type="password"
                autoComplete="current-password"
                required
                placeholder="Password"
              />
            </div>
          </div>
          {/* 
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <input
                id="remember-me"
                name="remember-me"
                type="checkbox"
                className="h-4 w-4 focus:ring-indigo-500 border-gray-300 rounded"
              />
              <label htmlFor="remember-me" className="ml-2 block text-sm">
                {' '}
                Remember me{' '}
              </label>
            </div>
          </div> */}

          <div>
            <Button type="submit" icon={<LockClosedIcon />}>
              Sign in
            </Button>
          </div>
        </Form>
      </Formik>
    </div>
  );
};

export default Login;
