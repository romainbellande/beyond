import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react';
import { config } from '@client/beyond/config';
import { Credentials, JwtResponse } from '../interfaces';

export const beyondApi = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({ baseUrl: config.apiUrl }),
  endpoints: (build) => ({
    register: build.mutation<any, Credentials>({
      query: (credentials) => ({
        url: 'auth/register',
        method: 'POST',
        body: credentials,
      }),
    }),
    login: build.mutation<JwtResponse, Credentials>({
      query: (credentials) => ({
        url: 'auth/login',
        method: 'POST',
        body: credentials,
      }),
    }),
  }),
});

export const { useRegisterMutation, useLoginMutation } = beyondApi;
