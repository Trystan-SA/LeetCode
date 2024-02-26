import esbuild from 'esbuild'
import childprocess from 'child_process'

let ServerProcess
export function SpawnProcess(){
    ServerProcess?.kill()
    ServerProcess = childprocess.fork('dist/bowling.cjs')
}

/** Editor Server */
esbuild.build({
    entryPoints: ['bownling.ts'],
    bundle: true,
    target: 'node14',
    platform: 'node',
    format: 'cjs',
    outfile: 'dist/bowling.cjs',
    watch: {
        onRebuild(error, result){
            if(error){console.error('Watch build failed : ', error)}
            else {
                SpawnProcess()
                console.log("Editor Server rebuilded")
            }
        }
    }
})
.then(()=>{SpawnProcess()})
.catch((err)=>{console.error(err);})