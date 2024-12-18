#ifndef __SkyPlane_H__
#define __SkyPlane_H__

#include "SdkWorld.h"

using namespace Ogre;
using namespace OgreBites;

class _OgreWorldClassExport World_SkyPlane : public SdkWorld
{
public:

    World_SkyPlane()
    {
        mInfo["Title"] = "Sky Plane";
        mInfo["Description"] = "Shows how to use skyplanes (fixed-distance planes used for backgrounds).";
        mInfo["Thumbnail"] = "thumb_skyplane.png";
        mInfo["Category"] = "Environment";
    }

protected:

    void setupContent() override
    {     
        // setup some basic lighting for our scene
        mSceneMgr->setAmbientLight(ColourValue(0.3, 0.3, 0.3));
        mSceneMgr->getRootSceneNode()
            ->createChildSceneNode(Vector3(20, 80, 50))
            ->attachObject(mSceneMgr->createLight());
        
        // create a skyplane 5000 units away, facing down, 10000 square units large, with 3x texture tiling
        mSceneMgr->setSkyPlane(true, Plane(0, -1, 0, 5000), "Examples/SpaceSkyPlane", 10000, 3);

        // and finally... omg it's a DRAGON!
        mSceneMgr->getRootSceneNode()->attachObject(mSceneMgr->createEntity("Dragon", "dragon.mesh"));

        // turn around and look at the DRAGON!
        mCameraNode->yaw(Degree(210));
        mCameraNode->pitch(Degree(-10));
    }
};

#endif
