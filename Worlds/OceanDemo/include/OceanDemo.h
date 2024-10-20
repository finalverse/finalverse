/*
-----------------------------------------------------------------------------
This source file is part of OGRE
    (Object-oriented Graphics Rendering Engine)
For the latest info, see http://www.ogre3d.org/

Copyright (c) 2000-2014 Torus Knot Software Ltd
Also see acknowledgements in Readme.html

You may use this sample code for anything you like, it is not covered by the
same license as the rest of the engine.
-----------------------------------------------------------------------------
*/

#ifndef _World_Ocean_H_
#define _World_Ocean_H_

#include "SdkWorld.h"
#include "WorldPlugin.h"
#include "MaterialControls.h"
#include "OgreBillboard.h"

using namespace Ogre;
using namespace OgreBites;

#define MINSPEED .150f
#define MOVESPEED 30
#define MAXSPEED 1.800f

#define CONTROLS_PER_PAGE 5

class _OgreWorldClassExport World_Ocean : public SdkWorld
{
public:
    World_Ocean();
protected:
    //Things from the frame listener
    Ogre::Vector3 mTranslateVector;
    int mSceneDetailIndex;
    float mUpdateFreq;
    bool mSpinLight;
    // just to stop toggles flipping too fast
    Ogre::TextureFilterOptions mFiltering;
    int mAniso;

    Ogre::SceneNode*      mMainNode;
    Ogre::Entity*         mOceanSurfaceEnt;

    size_t                mCurrentMaterial;
    size_t                mCurrentPage;
    size_t                mNumPages;
    Ogre::MaterialPtr     mActiveMaterial;
    Ogre::Pass*           mActivePass;
    Ogre::GpuProgramPtr   mActiveFragmentProgram;
    Ogre::GpuProgramPtr   mActiveVertexProgram;
    Ogre::GpuProgramParametersSharedPtr mActiveFragmentParameters;
    Ogre::GpuProgramParametersSharedPtr mActiveVertexParameters;
    Real                  mRotateSpeed;
    Slider* mShaderControls[CONTROLS_PER_PAGE];
    
    ShaderControlsContainer    mShaderControlContainer;
    MaterialControlsContainer mMaterialControlsContainer;
    
    void setupGUI();
    void setupScene();
    void setupContent() override;
    void cleanupContent() override;
    
    void sliderMoved(Slider* slider) override;
    void buttonHit(OgreBites::Button* button) override;
    void checkBoxToggled(CheckBox* box) override;
    void itemSelected(SelectMenu* menu) override;
    void changePage(int nextPage = -1);
    bool frameRenderingQueued(const FrameEvent& evt) override;
};

/**********************************************************************
  Static declarations
**********************************************************************/
// Lights
#define NUM_LIGHTS 1

// the light
Ogre::Light* mLights[NUM_LIGHTS];
// billboards for lights
Ogre::BillboardSet* mLightFlareSets[NUM_LIGHTS];
Ogre::Billboard* mLightFlares[NUM_LIGHTS];
// Positions for lights
Ogre::Vector3 mLightPositions[NUM_LIGHTS] =
{
    Ogre::Vector3(00, 400, 00)
};
// Base orientations of the lights
Ogre::Real mLightRotationAngles[NUM_LIGHTS] = { 35 };
Ogre::Vector3 mLightRotationAxes[NUM_LIGHTS] = {
    Ogre::Vector3::UNIT_X
};
// Rotation speed for lights, degrees per second
Ogre::Real mLightSpeeds[NUM_LIGHTS] = { 30};

// Colours for the lights
Ogre::ColourValue mDiffuseLightColours[NUM_LIGHTS] =
{
    Ogre::ColourValue(0.6, 0.6, 0.6)
};

Ogre::ColourValue mSpecularLightColours[NUM_LIGHTS] =
{
    Ogre::ColourValue(0.5, 0.5, 0.5)
};

// Which lights are enabled
bool mLightState[NUM_LIGHTS] =
{
    true
};

// the light nodes
Ogre::SceneNode* mLightNodes[NUM_LIGHTS];
// the light node pivots
Ogre::SceneNode* mLightPivots[NUM_LIGHTS];


World_Ocean::World_Ocean()
{
    mInfo["Title"] = "Ocean";
    mInfo["Description"] = "An example demonstrating ocean rendering using shaders.";
    mInfo["Thumbnail"] = "thumb_ocean.png";
    mInfo["Category"] = "Environment";
}
/*************************************************************************
                        World_Ocean Methods
*************************************************************************/
void World_Ocean::cleanupContent()
{
    MeshManager::getSingleton().remove("OceanSurface", ResourceGroupManager::DEFAULT_RESOURCE_GROUP_NAME);

    // get rid of the shared pointers before shutting down ogre or exceptions occur
    mActiveFragmentProgram.reset();
    mActiveFragmentParameters.reset();
    mActiveVertexProgram.reset();
    mActiveVertexParameters.reset();
    mActiveMaterial.reset();
}

//--------------------------------------------------------------------------
void World_Ocean::setupGUI(void)
{
    SelectMenu* selectMenu = mTrayMgr->createLongSelectMenu(
        TL_TOPLEFT, "MaterialSelectMenu", "Material", 300, 200, 5);
    
    for (size_t i=0; i<mMaterialControlsContainer.size(); i++)
    {
        selectMenu->addItem(mMaterialControlsContainer[i].getDisplayName());
    }

    mTrayMgr->createCheckBox(TL_TOPLEFT, "SpinLightButton", "Spin Light", 175)->setChecked(true);
    
    mTrayMgr->createButton(TL_TOPRIGHT, "PageButtonControl", "Page", 175);

    for (size_t i=0; i<CONTROLS_PER_PAGE; i++)
    {
        mShaderControls[i] = mTrayMgr->createThickSlider(TL_TOPRIGHT, 
            "ShaderControlSlider" + StringConverter::toString(i), "Control", 256, 80, 0, 1, 100);
    }

    selectMenu->selectItem(0);
    mTrayMgr->showCursor();

}
//--------------------------------------------------------------------------
void World_Ocean::setupContent(void)
{
    loadAllMaterialControlFiles(mMaterialControlsContainer);
    setupScene();
    setupGUI();
    
    // Position it at 500 in Z direction
    mCameraNode->setPosition(Ogre::Vector3(0,0,0));
    // Look back along -Z
    mCameraNode->lookAt(Ogre::Vector3(0,0,-300), Ogre::Node::TS_PARENT);
    mCamera->setNearClipDistance(1);

    setDragLook(true);
}

void World_Ocean::setupScene()
{
    // Set ambient light
    mSceneMgr->setAmbientLight(Ogre::ColourValue(0.3, 0.3, 0.3));
    mSceneMgr->setSkyBox(true, "SkyBox", 1000);

    mMainNode = mSceneMgr->getRootSceneNode()->createChildSceneNode();


    for (unsigned int i = 0; i < NUM_LIGHTS; ++i)
    {
        mLightPivots[i] = mSceneMgr->getRootSceneNode()->createChildSceneNode();
        mLightPivots[i]->rotate(mLightRotationAxes[i], Ogre::Angle(mLightRotationAngles[i]));
        // Create a light, use default parameters
        mLights[i] = mSceneMgr->createLight("Light" + Ogre::StringConverter::toString(i));
        mLights[i]->setDiffuseColour(mDiffuseLightColours[i]);
        mLights[i]->setSpecularColour(mSpecularLightColours[i]);
        mLights[i]->setVisible(mLightState[i]);
        //mLights[i]->setAttenuation(400, 0.1 , 1 , 0);
        // Attach light
        mLightPivots[i]->createChildSceneNode(mLightPositions[i])->attachObject(mLights[i]);
        // Create billboard for light
        mLightFlareSets[i] = mSceneMgr->createBillboardSet("Flare" + Ogre::StringConverter::toString(i));
        mLightFlareSets[i]->setMaterialName("LightFlare");
        mLightPivots[i]->attachObject(mLightFlareSets[i]);
        mLightFlares[i] = mLightFlareSets[i]->createBillboard(mLightPositions[i]);
        mLightFlares[i]->setColour(mDiffuseLightColours[i]);
        mLightFlareSets[i]->setVisible(mLightState[i]);
    }

    // move the camera a bit right and make it look at the knot
    mCameraNode->translate(Ogre::Vector3(50, 0, 100), Node::TS_LOCAL);
    mCameraNode->lookAt(Vector3(0, 0, 0), Node::TS_PARENT);

    // Define a plane mesh that will be used for the ocean surface
    Ogre::Plane oceanSurface;
    oceanSurface.normal = Ogre::Vector3::UNIT_Y;
    oceanSurface.d = 20;
    Ogre::MeshManager::getSingleton().createPlane("OceanSurface",
        Ogre::ResourceGroupManager::DEFAULT_RESOURCE_GROUP_NAME,
        oceanSurface,
        1000, 1000, 50, 50, true, 1, 1, 1, Ogre::Vector3::UNIT_Z);

    mOceanSurfaceEnt = mSceneMgr->createEntity( "OceanSurface", "OceanSurface" );
    mSceneMgr->getRootSceneNode()->createChildSceneNode()->attachObject(mOceanSurfaceEnt);
}


//--------------------------------------------------------------------------
void World_Ocean::sliderMoved(Slider* slider)
{
    using namespace Ogre;

    int sliderIndex = -1;
    for (int i=0; i<CONTROLS_PER_PAGE; i++)
    {
        if (mShaderControls[i] == slider)
        {
            sliderIndex = i;
            break;
        }
    }
    assert(sliderIndex != -1);

    size_t index = mCurrentPage * CONTROLS_PER_PAGE + sliderIndex;
    const ShaderControl& ActiveShaderDef = mMaterialControlsContainer[mCurrentMaterial].getShaderControl(index);

    float val = slider->getValue();
    
    if(mActivePass)
    {
        switch(ActiveShaderDef.ValType)
        {
            case GPU_VERTEX:
            case GPU_FRAGMENT:
                {
                    GpuProgramParametersSharedPtr activeParameters =
                        (ActiveShaderDef.ValType == GPU_VERTEX) ?
                            mActiveVertexParameters : mActiveFragmentParameters;

                    if(activeParameters)
                    {
                        activeParameters->_writeRawConstant(
                            ActiveShaderDef.PhysicalIndex + ActiveShaderDef.ElementIndex, val);
                    }
                }
                break;

            case MAT_SPECULAR:
                {
                    // get the specular values from the material pass
                    ColourValue OldSpec(mActivePass->getSpecular());
                    OldSpec[ActiveShaderDef.ElementIndex] = val;
                    mActivePass->setSpecular( OldSpec );
                }

                break;

            case MAT_DIFFUSE:
                {
                    // get the specular values from the material pass
                    ColourValue OldSpec(mActivePass->getDiffuse());
                    OldSpec[ActiveShaderDef.ElementIndex] = val;
                    mActivePass->setDiffuse( OldSpec );
                }
                break;

            case MAT_AMBIENT:
                {
                    // get the specular values from the material pass
                    ColourValue OldSpec(mActivePass->getAmbient());
                    OldSpec[ActiveShaderDef.ElementIndex] = val;
                    mActivePass->setAmbient( OldSpec );
                }
                break;

            case MAT_SHININESS:
                // get the specular values from the material pass
                mActivePass->setShininess( val );
                break;

            case MAT_EMISSIVE:
                break;
        }
    }
}

//--------------------------------------------------------------------------
void World_Ocean::changePage(int pageNum /* = -1 : toggle */)
{
    if (mMaterialControlsContainer.empty()) return;
    mCurrentPage = (pageNum == -1) ? (mCurrentPage+1) % mNumPages : pageNum;

    static char pageText[64];
    sprintf(pageText, "Parameters %zu / %zu", mCurrentPage+1, mNumPages);
    static_cast<OgreBites::Button*>(mTrayMgr->getWidget("PageButtonControl"))->setCaption(pageText);

    if(mActiveMaterial && !mActiveMaterial->getSupportedTechniques().empty())
    {
        Ogre::Technique* currentTechnique = mActiveMaterial->getSupportedTechniques().front();
        if(currentTechnique)
        {
            mActivePass = currentTechnique->getPass(0);
            if(mActivePass)
            {
                if (mActivePass->hasFragmentProgram())
                {
                    mActiveFragmentProgram = mActivePass->getFragmentProgram();
                    mActiveFragmentParameters = mActivePass->getFragmentProgramParameters();
                }
                if (mActivePass->hasVertexProgram())
                {
                    mActiveVertexProgram = mActivePass->getVertexProgram();
                    mActiveVertexParameters = mActivePass->getVertexProgramParameters();
                }

                size_t activeControlCount = mMaterialControlsContainer[mCurrentMaterial].getShaderControlCount();
                
                size_t startControlIndex = mCurrentPage * CONTROLS_PER_PAGE;
                int numControls = static_cast<int>(activeControlCount - startControlIndex);
                if (numControls <= 0)
                {
                    mCurrentPage = 0;
                    startControlIndex = 0;
                    numControls = (int)activeControlCount;
                }
                
                for (size_t i=0; i<CONTROLS_PER_PAGE; i++)
                {
                    Slider* shaderControlSlider = mShaderControls[i];
                    if (i < (size_t)numControls)
                    {
                        shaderControlSlider->show();
                        size_t controlIndex = startControlIndex + i;
                        const ShaderControl& ActiveShaderDef = mMaterialControlsContainer[mCurrentMaterial].getShaderControl(controlIndex);
                        shaderControlSlider->setRange(ActiveShaderDef.MinVal, ActiveShaderDef.MaxVal, 50, false);
                        shaderControlSlider->setCaption(ActiveShaderDef.Name);

                        float uniformVal = 0.0;
                        switch(ActiveShaderDef.ValType)
                        {
                            case GPU_VERTEX:
                            case GPU_FRAGMENT:
                                {
                                    Ogre::GpuProgramParametersSharedPtr activeParameters =
                                        (ActiveShaderDef.ValType == GPU_VERTEX) ?
                                            mActiveVertexParameters : mActiveFragmentParameters;
                                    if(activeParameters)
                                    {
                                        // use param name to get index : use appropriate parameters ptr
                                        const Ogre::GpuConstantDefinition& def = 
                                            activeParameters->getConstantDefinition(ActiveShaderDef.ParamName);
                                        ActiveShaderDef.PhysicalIndex = def.physicalIndex;
                                        // use index to get RealConstantEntry
                                        const float* pFloat = activeParameters->getFloatPointer(ActiveShaderDef.PhysicalIndex);
                                        // set position of ScrollWidget as param value
                                        uniformVal = pFloat[ActiveShaderDef.ElementIndex];
                                    }
                                }
                                break;

                            case MAT_SPECULAR:
                                {
                                    // get the specular values from the material pass

                                    Ogre::ColourValue OldSpec(mActivePass->getSpecular());
                                    uniformVal = OldSpec[ActiveShaderDef.ElementIndex];
                                }
                                break;

                            case MAT_DIFFUSE:
                                {
                                    // get the diffuse values from the material pass

                                    Ogre::ColourValue OldSpec(mActivePass->getDiffuse());
                                    uniformVal = OldSpec[ActiveShaderDef.ElementIndex];
                                }
                                break;

                            case MAT_AMBIENT:
                                {
                                    // get the ambient values from the material pass

                                    Ogre::ColourValue OldSpec(mActivePass->getAmbient());
                                    uniformVal = OldSpec[ActiveShaderDef.ElementIndex];
                                }
                                break;

                            case MAT_SHININESS:
                                {
                                    // get the ambient values from the material pass
                                    uniformVal = mActivePass->getShininess();
                                }

                                break;

                            case MAT_EMISSIVE:
                                {
                                    // get the ambient values from the material pass

                                    //ColourValue OldSpec(mActivePass->gete());
                                    //activeScrollWidget->setScrollPosition( OldSpec.val[ActiveShaderDef->ElementIndex] );
                                }
                                break;
                        }
                        shaderControlSlider->setValue(uniformVal);
                        
                    }
                    else
                    {
                        shaderControlSlider->hide();
                    }
                }
            }
        }
    }

}
void World_Ocean::itemSelected(SelectMenu *menu)
{
    //Only one selection menu - the material one
    mCurrentMaterial = menu->getSelectionIndex();
    mActiveMaterial = Ogre::MaterialManager::getSingleton().getByName( mMaterialControlsContainer[mCurrentMaterial].getMaterialName() );
    mActiveMaterial->load();
    size_t numShaders = mMaterialControlsContainer[mCurrentMaterial].getShaderControlCount();
    mNumPages = (numShaders / CONTROLS_PER_PAGE) + (numShaders % CONTROLS_PER_PAGE == 0 ? 0 : 1);
    changePage(0);

    if (mOceanSurfaceEnt)
        mOceanSurfaceEnt->setMaterialName(mMaterialControlsContainer[mCurrentMaterial].getMaterialName());
}
bool World_Ocean::frameRenderingQueued(const FrameEvent& evt)
{
    mRotateSpeed = evt.timeSinceLastFrame * 20;
    if(mSpinLight)
    {
        mLightPivots[0]->rotate(mLightRotationAxes[0], Ogre::Angle(mRotateSpeed * 2.0f));
    }
    return SdkWorld::frameRenderingQueued(evt);
}
void World_Ocean::buttonHit(OgreBites::Button* button)
{
    //Only one button - change page
    changePage();
}

void World_Ocean::checkBoxToggled(CheckBox* cb)
{
    //Only one checkbox
    mSpinLight = cb->isChecked();
}

#endif  // end _World_Ocean_H_
