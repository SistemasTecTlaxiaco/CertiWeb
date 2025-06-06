<!-- <script>
// import { Server, TransactionBuilder, Operation, Asset } from '@stellar/stellar-sdk';
// import { Server } from 'stellar-sdk';
import { Server, TransactionBuilder, Operation, Asset } from '@stellar/stellar-sdk';

export default {
  name: 'App',
  data() {
    return {
      walletConnected: false,
      publicKey: '',
      balance: '',
      firstName: '',  
      lastName: '',
      documentId: '',
      title: '',
      status: '',
      contractId: 'CAX2TPLPGSAVFNSCEHBJYEYQTBZ7KW2DPULPZXJNIEV7YQ6445EQX7JQ',
      server: new Server('https://horizon-testnet.stellar.org'),
      networkPassphrase: 'Test SDF Network ; September 2015'
    };
  },
  methods: {
    // tus métodos (connectFreighter, registrarDocumento, etc.)
  }
};
</script> -->

<!-- <script>
import { Server, TransactionBuilder, Operation, Asset } from '@stellar/stellar-sdk';

export default {
  name: 'App',
  data() {
    return {
      walletConnected: false,
      publicKey: '',
      balance: '',
      firstName: '',
      lastName: '',
      documentId: '',
      title: '',
      status: '',
      contractId: 'CAX2TPLPGSAVFNSCEHBJYEYQTBZ7KW2DPULPZXJNIEV7YQ6445EQX7JQ',
      server: new Server('https://horizon-testnet.stellar.org'),
      networkPassphrase: 'Test SDF Network ; September 2015'
    };
  },
  methods: {
    // tus métodos aquí (connectFreighter, registrarDocumento, etc.)
  }
};
</script> -->
<!-- <script>
import { isConnected, getPublicKey, signTransaction } from "@stellar/freighter-api";

export default {
  name: 'App',
  data() {
    return {
      walletConnected: false,
      publicKey: '',
      balance: '',
      contractId: 'CAX2TPLPGSAVFNSCEHBJYEYQTBZ7KW2DPULPZXJNIEV7YQ6445EQX7JQ',
      networkPassphrase: 'Test SDF Network ; September 2015'
    };
  },
  methods: {
    async connectFreighter() {
      try {
        const connected = await isConnected();
        if (!connected) {
          // Lanza el modal de conexión de Freighter
          const pubKey = await getPublicKey();
          this.walletConnected = true;
          this.publicKey = pubKey;
          console.log("Wallet conectada:", pubKey);
        } else {
          const pubKey = await getPublicKey();
          this.walletConnected = true;
          this.publicKey = pubKey;
          console.log("Wallet ya estaba conectada:", pubKey);
        }
      } catch (error) {
        console.error("Error al conectar con Freighter:", error);
        alert("No se pudo conectar con Freighter. ¿Está instalada la extensión?");
      }
    }
  }
};

export default {
  name: 'App',
  data() {
    return {
      walletConnected: false,
      publicKey: '',
      balance: '',
      firstName: '',  
      lastName: '',
      documentId: '',
      title: '',
      status: '',
      contractId: 'CAX2TPLPGSAVFNSCEHBJYEYQTBZ7KW2DPULPZXJNIEV7YQ6445EQX7JQ',
      server: null,
      networkPassphrase: 'Test SDF Network ; September 2015'
    };
  },
  methods: {
    connectFreighter() {
      console.log('Conectando wallet...');
      // lógica para conectar la wallet
    },
    registrarDocumento() {
      console.log('Registrando documento...');
      // lógica para registrar documento
    }
  }
};
import { Contract, Server } from 'soroban-client';

const server = new Server('https://rpc-tesnet.stellar.org');
const contract = new Contract('<contract-id>');

async function registrarDocumento(id_documento, titulo, estado, fecha) {
  const tx = await contract.call('registrar_documento', {
    id_documento,
    titulo,
    estado,
    fecha
  });
  const response = await server.submitTransaction(tx);
  console.log(response);
}

</script> -->



<!-- <style>
.wallet-info {
  margin-bottom: 20px;
  padding: 10px;
  background-color: #f8f9fa;
  border-radius: 5px;
}

.center {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style> -->
<script>
// import { isConnected, getPublicKey, signTransaction } from "@stellar/freighter-api";
import { isConnected, getUserInfo, signTransaction } from "@stellar/freighter-api";

import { Contract, Server, TransactionBuilder, Networks, BASE_FEE, Keypair, xdr } from "soroban-client";

export default {
  name: 'App',
  data() {
    return {
      walletConnected: false,
      publicKey: '',
      balance: '',
      firstName: '',
      lastName: '',
      documentId: '',
      title: '',
      status: '',
      contractId: 'CAX2TPLPGSAVFNSCEHBJYEYQTBZ7KW2DPULPZXJNIEV7YQ6445EQX7JQ',
      networkPassphrase: 'Test SDF Network ; September 2015',
      server: new Server('https://rpc-futurenet.stellar.org') // Se recomienda usar Futurenet para pruebas
    };
  },
  methods: {
    async connectFreighter() {
  try {
    const { publicKey } = await getUserInfo();
    this.walletConnected = true;
    this.publicKey = publicKey;
    console.log("Wallet conectada:", publicKey);
  } catch (error) {
    console.error("Error al conectar con Freighter:", error);
    alert("No se pudo conectar con Freighter. ¿Está instalada la extensión?");
  }
},

    async registrarDocumento() {
      if (!this.walletConnected) {
        alert("Conecta tu wallet antes de registrar un documento.");
        return;
      }

      try {
        // Construir el contrato
        const contract = new Contract(this.contractId);

        // Crear una transacción para invocar la función del contrato
        const sourceAccount = await this.server.getAccount(this.publicKey);
        const transaction = new TransactionBuilder(sourceAccount, {
          fee: BASE_FEE,
          networkPassphrase: this.networkPassphrase
        })
          .addOperation(contract.call(
            'registrar_documento',
            {
              id_documento: this.documentId,
              titulo: this.title,
              estado: this.status
            }
          ))
          .setTimeout(30)
          .build();

        // Serializar la transacción para Freighter
        const xdrTx = transaction.toXDR();

        // Pedir a Freighter que firme la transacción
        const signedXDR = await signTransaction(xdrTx, {
          network: this.networkPassphrase
        });

        // Enviar la transacción firmada a la red
        const txResponse = await this.server.submitTransaction(xdr.TransactionEnvelope.fromXDR(signedXDR, 'base64'));
        console.log("Transacción enviada con éxito:", txResponse);
        alert("Documento registrado correctamente en la blockchain.");

      } catch (error) {
        console.error("Error al registrar documento:", error);
         alert("Error al registrar el documento.");
        

      }
    }
  }
};
</script>

<template>
  <div class="container">
    <div class="row">
      <div class="card mt-4">
        <h1 style="text-align: center;">Gestor de Documentos en Stellar</h1>
        <br>
        <!-- Botón para conectar Freighter -->
        <button class="btn btn-primary" @click="connectFreighter" v-if="!walletConnected">
          Conectar Freighter Wallet
        </button>

        <div v-if="walletConnected" class="wallet-info">
          <p>Conectado como: {{ publicKey }}</p>
          <p>Balance: {{ balance }} XLM</p>
        </div>
        <br><br>
        <div class="input-group center">
          <span class="input-group-text center">Nombre completo:</span>
          <input type="text" v-model="firstName" placeholder="Nombre" class="form-control">
          <input type="text" v-model="lastName" placeholder="Apellido" class="form-control">
        </div>
        <br><br>
        <div class="input-group mb-3">
          <span class="input-group-text" id="inputGroup-sizing-default">ID Documento</span>
          <input type="text" v-model="documentId" class="form-control">
        </div>
        <br><br>
        <div class="input-group mb-3">
          <span class="input-group-text" id="inputGroup-sizing-default">Título</span>
          <input type="text" v-model="title" class="form-control">
        </div>
        <br><br>
        <div class="input-group mb-3">
          <label class="input-group-text" for="inputGroupSelect01">Estado</label>
          <select class="form-select" id="inputGroupSelect01" v-model="status">
            <option disabled value="">Selecciona un estado</option>
            <option value="Titulado">Titulado</option>
            <option value="Activo">Activo</option>
            <option value="Reprobado">Reprobado</option>
          </select>
        </div>
        <button class="btn btn-warning center" @click="registrarDocumento">
          Registrar Documento
        </button>
        <br>
      </div>
    </div>
  </div>
</template>

<style>
.wallet-info {
  margin-bottom: 20px;
  padding: 10px;
  background-color: #f8f9fa;
  border-radius: 5px;
}

.center {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
