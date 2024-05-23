import { sleep } from "k6";
import http from 'k6/http';

export const options = {
    vus: 30,
    iterations: 300    
};

export default function () {
    http.get('http://192.168.3.91:8888/api/v1/users');
    sleep(1);
  }