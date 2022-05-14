import { FC } from 'react';
import { Button } from '@client/ui';
import { LockClosedIcon } from '@heroicons/react/solid';
import { Formik, Form, ErrorMessage, Field } from 'formik';
import { Credentials } from '@client/beyond/interfaces';
import * as Yup from 'yup';
import { useRegisterMutation } from '@client/beyond/store/api';

const validationSchema = Yup.object().shape({
  username: Yup.string().required(),
  password: Yup.string().min(8).required(),
});

const Register: FC = () => {
  const [register, result] = useRegisterMutation();

  const initialValues: Credentials = {
    username: '',
    password: '',
  };

  const onSubmit = async (credentials: Credentials) => {
    await register(credentials);
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
          Register an account
        </h2>
        <p className="mt-2 text-center text-sm text-gray-600">
          Or
          <a
            href="#"
            className="font-medium text-indigo-600 hover:text-indigo-500"
          >
            {' '}
            login to your account{' '}
          </a>
        </p>
      </div>
      <Formik
        initialValues={initialValues}
        validationSchema={validationSchema}
        onSubmit={onSubmit}
      >
        <Form className="mt-8 space-y-6">
          {/* <Input type="hidden" name="remember" value="true" /> */}
          <div className="rounded-md shadow-sm space-y-4">
            <div>
              <label htmlFor="username" className="sr-only">
                Email address
              </label>
              <Field
                id="username"
                className="field"
                name="username"
                required
                placeholder="Username"
              />
              <div className="error-message">
                <ErrorMessage name="username" />
              </div>
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
                required
                placeholder="Password"
              />
              <div className="error-message">
                <ErrorMessage name="password" />
              </div>
            </div>
          </div>

          <Button type="submit" icon={<LockClosedIcon />}>
            Register
          </Button>
        </Form>
      </Formik>
    </div>
  );
};

export default Register;
