import { sleep } from "k6";
import http from 'k6/http';

export const options = {
    vus: 20,
    iterations: 200    
};

export default function () {
    http.get('http://192.168.3.91:8888/api/v2/users');
    sleep(1);
  }