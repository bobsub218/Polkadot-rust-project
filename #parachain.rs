
[package]
name = "p3_chain"
version = "0.1.0"
[dependencies]
polkadot-parachain = { path = "<path to polkadot/parachain>" }
polkadot-collator = { path = "<path to polkadot/collator>" }
polkadot-primitives = { path = "<path to polkadot/primitives>" }


//P1 LOGICA PARACHAIN

//creare uan logica semplice per introdurre alcuni nuovi tipi di dati
//includiamo l'API
//includiamo le interfacce e i tipi di dati


#[macro_use]
extern crate parity_codec_derive;
extern crate parity_codec;
extern crate polkadot_parachain as parachain;
extern crate tiny_keccak;


//useremo l'ereditarietò di dati Polkadot standard per ottenere proprietà convenienti per l'hashing e la codifica
// È importante utilizzare i nomi offerte da Polkadot per i tipi principali, 
///poiché sostituiamo semplicemente i tipi standard con quelli personalizzati
#[derive(Default, Clone, Hash, Eq, PartialEq, Encode, Decode)]
pub struct HeadData {
    pub block_number: u64,
    pub parent_hash: [u8; 32],
    pub state_hash: [u8; 32],

}
#[derive(Default, Clone, Encode, Decode)]
pub struct BlockData {
    pub data_hash: [u8; 32],
    pub some_data: IotData,

} 


// è il nostro tipo personalizzato da archiviare on-chain
#[derive(Default, Clone, Encode, Decode)]
pub struct IotData {
    pub iotId: u64,
    pub parameter: [char; 20],
    pub value: u64,
} 



///implementiamo qualsiasi logica on-chain 
/// creiamo una funzione per l'esecuzione di logica personalizzata
/// creiamo una funzione per convalidare dei blocchi e nuove intestazioni 
pub fn produce_valid_header(
    parent_hash: [u8; 32],
    parent_head: HeadData,
    block_data: &BlockData,
) - Result<HeadData, Debug> {
    ///All appropriate checks should be placed here
    debug_assert_eq!(parent_hash, parent_head.hash());
     
    ///Here may be state_hash check for many block_data parts
    
    ///Block data check
    let calculated_hash  = ::tiny_keccak::keccak256( block_data.some_data[..] )
    if calculated_hash != block_data.data_hash {
        return Err(Debug);
    }
     
    let new_state_hash = ::tiny_keccak::keccak256( block_data.encode().as_slice() );
    Ok(HeadData {
        block_number: parent_head.block_number + 1,
        parent_hash,
        state_hash: new_state_hash,
    })
}
pub fn execute(
    parent_hash: [u8; 32],
    parent_head: HeadData,
    some_data: &SomeData,
)   Result<HeadData, Debug> {
    let block_data: BlockData;
     
    ///Custom operations: process some_data into BlockData
    match produce_valid_header( parent_hash, parent_head, &block_data ) {
        Ok(new_head) => new_head,
        Err(_) => Err(Debug),
    } 


///P2 INTERFACCIA WASM
/// 
/// sviluppare un'interfaccia per la convalida dei blocchi di parachea.
 
/// Può contenere una funzione di gestione degli errori personalizzata, ma deve contenere la funzione di convalida.
//custom_parachain è la funzionalità implementata nel passaggio precedente
use parachain::ValidationResult;
use parachain::codec::{Encode, Decode};
use custom_parachain::{HeadData, BlockData}; 


/// implementiamo la funzione
pub extern fn validate(offset: usize, len: usize) -> usize {
    let params = unsafe { ::parachain::wasm_api::load_params(offset, len) };
    let parent_head = HeadData::decode(&mut &params.parent_head[..])
        .expect("invalid parent head format.");
    let block_data = BlockData::decode(&mut &params.block_data[..])
        .expect("invalid block data format.");
    let parent_hash = ::tiny_keccak::keccak256( &params.parent_head[..]);
    match ::custom_parachain::produce_valid_header( parent_hash, parent_head, &block_data) {
        Ok(new_head) => parachain::wasm_api::write_result(
            ValidationResult { head_data: new_head.encode() }
        ),
        Err(_) => panic!("validation failure"),
    }
} 



/// P3 NODO FASCICOLATORE


///L'ultima cosa da implementare è un nodo di confronto. 
/// Dovrebbe richiamare funzioni dalla logica paracatena per produrre blocchi per la catena di relè.

///Includi l'API parachain, i tipi di dati e le interfacce
use custom_parachain::{HeadData as CustomHead, BlockData as CustomBody, IotData};
use primitives::parachain::{HeadData, BlockData, Id as ParaId, Message, Extrinsic};
use collator::{InvalidHead, ParachainContext, VersionInfo}; 

///I dati del blocco Genesis sono inseriti anche nel codice del fascicolatore:
const GENESIS: CustomHead = CustomHead {
    block_number: 0,
    parent_hash: [0; 32],
    state_hash: [some_value],///calculated initial hash value
};
const GENESIS_BODY: CustomBody = CustomBody {
    data_hash: [some_value],
    some_data: IotData {
        iotId: 0,
        parameter: ['n', 'o', 'n', 'e'],
        value: 0,
    }
}




///La nostra logica parachain dovrebbe derivare dalla logica comune e interagire con la blockchain del database
#[derive(Clone)]
struct CustomContext {
    db: Arc<Mutex<HashMap<CustomHead, CustomBody>>>
}

impl ParachainContext for CustomContext {
    fn produce_candidate<I: IntoIterator<Item=(ParaId, Message)>>(
        &  self,
        last_head: HeadData,
        ingress: I
    )   Result<(BlockData, HeadData, Extrinsic), InvalidHead>

    {
        ///Get parent_head from last_head
        ///Get previous block_data from the database by adder_head or GENESIS_BODY

        let next_head = ::custom_paracain::execute( parent_head.hash(), parent_head, &next_body );
        let encoded_head = HeadData(next_head.encode());
        let encoded_body = BlockData(next_body.encode());
        ///Put new block to the database
        db.insert(next_head.clone(), next_body);
        Ok((encoded_body, encoded_head, Extrinsic { outgoing_messages: Vec::new() }))
    }
}

///La funzione principale deve contenere solo valori genesis e una chiamata al fascitore::run_collator:
fn main() {
    let id: ParaId = 100.into();

    let encoded = GENESIS.encode();
    let context = CustomContext {
        db: Arc::new(Mutex::new(HashMap::new())),
    };

    let res = ::collator::run_collator(
        context,
        id
        exit_function,
        key_function,
        ::std::env::args(),
        VersionInfo {
           ......
        }
    )
} 

///Ora il paracahin è pronto per la compilazione e la distribuzione.
/// Dalla cartella Wasm vengono compilate nel BLOB Wasm da wasm-gc. 
/// Questo codice Wasm può essere distribuito su Polkadot usando le istruzioni fornite sul wiki Polkadot. 
/// Avrai bisogno solo delle informazioni codificate genesis e di alcuni DOT per la distribuzione dei parachain. 
/// Successivamente, è possibile compilare il codice del fascicolatore, accedere alla catena e visualizzare il blocco su Polkaskan.
