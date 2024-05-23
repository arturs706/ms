import { sleep } from "k6";
import http from 'k6/http';

export const options = {
    vus: 10,
    iterations: 100    
};

export default function () {
    http.get('http://192.168.3.91:9948/api/v2/users');
    sleep(1);
}