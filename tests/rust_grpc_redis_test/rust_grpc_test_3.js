import grpc from "k6/net/grpc";
import { sleep } from "k6";
    
    const client = new grpc.Client();
    client.load(["../../protos"], "staffusers.proto");
    
    export const options = {
        vus: 30,
        iterations: 300    
    };
    
    
    export default () => {
        client.connect("192.168.3.91:3000", {
            plaintext: true,
        });
        const data = { message: "" };
        client.invoke("staffusers.StaffUsers/GetAllUsers", data);
        client.close();
        sleep(1);
    };
