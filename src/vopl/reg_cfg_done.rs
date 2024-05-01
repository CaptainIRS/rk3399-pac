#[doc = "Register `REG_CFG_DONE` reader"]
pub type R = crate::R<RegCfgDoneSpec>;
#[doc = "Register `REG_CFG_DONE` writer"]
pub type W = crate::W<RegCfgDoneSpec>;
#[doc = "Field `REG_LOAD_EN` writer - vop register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the register config finish, writing this register to enable the\n\ncopyright of the mirror register to real register. Then register would\n\nbe updated at the start of every frame."]
pub type RegLoadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LOAD_WIN0_EN` reader - vop win0 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win0 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadWin0EnR = crate::BitReader;
#[doc = "Field `REG_LOAD_WIN0_EN` writer - vop win0 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win0 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadWin0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LOAD_WIN1_EN` reader - vop win1 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win1 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadWin1EnR = crate::BitReader;
#[doc = "Field `REG_LOAD_WIN1_EN` writer - vop win1 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win1 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadWin1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LOAD_WIN2_EN` reader - vop win2 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win2 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadWin2EnR = crate::BitReader;
#[doc = "Field `REG_LOAD_WIN2_EN` writer - vop win2 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win2 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadWin2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LOAD_WIN3_EN` reader - vop win3 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win3 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadWin3EnR = crate::BitReader;
#[doc = "Field `REG_LOAD_WIN3_EN` writer - vop win3 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win3 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadWin3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LOAD_HWC_EN` reader - vop hwc register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the hwc register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadHwcEnR = crate::BitReader;
#[doc = "Field `REG_LOAD_HWC_EN` writer - vop hwc register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the hwc register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadHwcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LOAD_IEP_EN` reader - vop iep register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the iep register config finish(only 2 signals\n\ndirect_path_en,direct_path_layer_sel ), writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadIepEnR = crate::BitReader;
#[doc = "Field `REG_LOAD_IEP_EN` writer - vop iep register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the iep register config finish(only 2 signals\n\ndirect_path_en,direct_path_layer_sel ), writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadIepEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LOAD_FBDC_EN` reader - vop fbdc register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the fbdc register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadFbdcEnR = crate::BitReader;
#[doc = "Field `REG_LOAD_FBDC_EN` writer - vop fbdc register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the fbdc register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadFbdcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LOAD_SYS_EN` reader - vop system register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the system register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadSysEnR = crate::BitReader;
#[doc = "Field `REG_LOAD_SYS_EN` writer - vop system register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the system register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
pub type RegLoadSysEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` reader - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_MASK` writer - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 1 - vop win0 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win0 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    pub fn reg_load_win0_en(&self) -> RegLoadWin0EnR {
        RegLoadWin0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - vop win1 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win1 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    pub fn reg_load_win1_en(&self) -> RegLoadWin1EnR {
        RegLoadWin1EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - vop win2 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win2 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    pub fn reg_load_win2_en(&self) -> RegLoadWin2EnR {
        RegLoadWin2EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - vop win3 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win3 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    pub fn reg_load_win3_en(&self) -> RegLoadWin3EnR {
        RegLoadWin3EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - vop hwc register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the hwc register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    pub fn reg_load_hwc_en(&self) -> RegLoadHwcEnR {
        RegLoadHwcEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - vop iep register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the iep register config finish(only 2 signals\n\ndirect_path_en,direct_path_layer_sel ), writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    pub fn reg_load_iep_en(&self) -> RegLoadIepEnR {
        RegLoadIepEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - vop fbdc register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the fbdc register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    pub fn reg_load_fbdc_en(&self) -> RegLoadFbdcEnR {
        RegLoadFbdcEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - vop system register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the system register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    pub fn reg_load_sys_en(&self) -> RegLoadSysEnR {
        RegLoadSysEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    pub fn write_mask(&self) -> WriteMaskR {
        WriteMaskR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - vop register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the register config finish, writing this register to enable the\n\ncopyright of the mirror register to real register. Then register would\n\nbe updated at the start of every frame."]
    #[inline(always)]
    #[must_use]
    pub fn reg_load_en(&mut self) -> RegLoadEnW<RegCfgDoneSpec> {
        RegLoadEnW::new(self, 0)
    }
    #[doc = "Bit 1 - vop win0 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win0 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    #[must_use]
    pub fn reg_load_win0_en(&mut self) -> RegLoadWin0EnW<RegCfgDoneSpec> {
        RegLoadWin0EnW::new(self, 1)
    }
    #[doc = "Bit 2 - vop win1 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win1 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    #[must_use]
    pub fn reg_load_win1_en(&mut self) -> RegLoadWin1EnW<RegCfgDoneSpec> {
        RegLoadWin1EnW::new(self, 2)
    }
    #[doc = "Bit 3 - vop win2 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win2 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    #[must_use]
    pub fn reg_load_win2_en(&mut self) -> RegLoadWin2EnW<RegCfgDoneSpec> {
        RegLoadWin2EnW::new(self, 3)
    }
    #[doc = "Bit 4 - vop win3 register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the win3 register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    #[must_use]
    pub fn reg_load_win3_en(&mut self) -> RegLoadWin3EnW<RegCfgDoneSpec> {
        RegLoadWin3EnW::new(self, 4)
    }
    #[doc = "Bit 5 - vop hwc register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the hwc register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    #[must_use]
    pub fn reg_load_hwc_en(&mut self) -> RegLoadHwcEnW<RegCfgDoneSpec> {
        RegLoadHwcEnW::new(self, 5)
    }
    #[doc = "Bit 6 - vop iep register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the iep register config finish(only 2 signals\n\ndirect_path_en,direct_path_layer_sel ), writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    #[must_use]
    pub fn reg_load_iep_en(&mut self) -> RegLoadIepEnW<RegCfgDoneSpec> {
        RegLoadIepEnW::new(self, 6)
    }
    #[doc = "Bit 7 - vop fbdc register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the fbdc register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    #[must_use]
    pub fn reg_load_fbdc_en(&mut self) -> RegLoadFbdcEnW<RegCfgDoneSpec> {
        RegLoadFbdcEnW::new(self, 7)
    }
    #[doc = "Bit 8 - vop system register config done flag\n\nIn the first setting of the register, the new value was saved into the\n\nmirror register.\n\nWhen all the system register config finish, writing this register to\n\nenable the copyright of the mirror register to real register. Then\n\nregister would be updated at the start of every frame."]
    #[inline(always)]
    #[must_use]
    pub fn reg_load_sys_en(&mut self) -> RegLoadSysEnW<RegCfgDoneSpec> {
        RegLoadSysEnW::new(self, 8)
    }
    #[doc = "Bits 16:31 - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<RegCfgDoneSpec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Register config done flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_cfg_done::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_cfg_done::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCfgDoneSpec;
impl crate::RegisterSpec for RegCfgDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cfg_done::R`](R) reader structure"]
impl crate::Readable for RegCfgDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cfg_done::W`](W) writer structure"]
impl crate::Writable for RegCfgDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_CFG_DONE to value 0"]
impl crate::Resettable for RegCfgDoneSpec {
    const RESET_VALUE: u32 = 0;
}
