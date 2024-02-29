{-# LANGUAGE MultiWayIf    #-}
{-# LANGUAGE UnicodeSyntax #-}

import           Hake

main ∷ IO ()
main = hake $ do

  "clean | clean the project" ∫
    cargo ["clean"] ?> removeDirIfExists targetPath

  "update | update dependencies" ∫ cargo ["update"]

  twilightExecutable ♯
    cargo <| "build" : buildFlagsTwilight False

  "fat | build twilight  with fat LTO" ∫
       cargo <| "build" : buildFlagsTwilight True

  "install | install to system" ◉ [ "fat" ] ∰
    cargo <| "install" : buildFlagsTwilight True

  "test | build and test" ◉ [twilightExecutable] ∰ do
    cargo ["test"]
    cargo ["clippy"]
    rawSystem twilightExecutable ["--version"]
      >>= checkExitCode

  "restart | restart services" ◉ [ twilightExecutable ] ∰
    systemctl ["restart", appNameTwilight]

  "run | run twilight" ◉ [ twilightExecutable ] ∰ do
    cargo . (("run" : buildFlagsTwilight False) ++) . ("--" :) =<< getHakeArgs

 where
  appNameTwilight ∷ String
  appNameTwilight = "twilight"

  targetPath ∷ FilePath
  targetPath = "target"

  buildPath ∷ FilePath
  buildPath = targetPath </> "release"

  twilightFeatures ∷ [String]
  twilightFeatures = [ ]

  fatArgs ∷ [String]
  fatArgs = [ "--profile"
            , "fat-release" ]

  buildFlagsTwilight ∷ Bool -> [String]
  buildFlagsTwilight fat =
    let defaultFlags = [ "-p", appNameTwilight
                       , "--release", "--features"
                       , intercalate "," twilightFeatures ]
    in if fat then defaultFlags ++ fatArgs
              else defaultFlags

  twilightExecutable ∷ FilePath
  twilightExecutable = buildPath </> appNameTwilight
