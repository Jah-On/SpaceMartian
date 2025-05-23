using Flax.Build;
using Flax.Build.NativeCpp;
using System.IO;

public class Game : GameModule
{
    /// <inheritdoc />
    public override void Init()
    {
        base.Init();

        // C#-only scripting
        BuildNativeCode = false;
    }

    /// <inheritdoc />
    public override void Setup(BuildOptions options)
    {
        base.Setup(options);

        options.ScriptingAPI.IgnoreMissingDocumentationWarnings = true;
        options.ScriptingAPI.SystemReferences.Add("System.Runtime.Serialization");
        options.ScriptingAPI.SystemReferences.Add("System.Runtime.Serialization.Primitives");
        options.ScriptingAPI.FileReferences.Add(
            Path.Combine(FolderPath, "..", "..", "Content", "SpacetimeDB.BSATN.Runtime.dll")
        );
        options.ScriptingAPI.Analyzers.Add(
           Path.Combine(FolderPath, "..", "..", "Content", "SpacetimeDB.BSATN.Codegen.dll")
       );
        options.ScriptingAPI.FileReferences.Add(
            Path.Combine(FolderPath, "..", "..", "Content", "SpacetimeDB.ClientSDK.dll")
        );
        options.ScriptingAPI.CSharpNullableReferences = CSharpNullableReferences.Enable;
        // Here you can modify the build options for your game module
        // To reference another module use: options.PublicDependencies.Add("Audio");
        // To add C++ define use: options.PublicDefinitions.Add("COMPILE_WITH_FLAX");
        // To learn more see scripting documentation.
    }
}
