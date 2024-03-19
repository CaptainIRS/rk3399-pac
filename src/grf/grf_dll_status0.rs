#[doc = "Register `GRF_DLL_STATUS0` reader"]
pub type R = crate::R<GrfDllStatus0Spec>;
#[doc = "Register `GRF_DLL_STATUS0` writer"]
pub type W = crate::W<GrfDllStatus0Spec>;
#[doc = "Field `PVTM_CORE_L_FREQ_DONE` reader - pd_core_l pvtm frequency calculate done\n\nstutus"]
pub type PvtmCoreLFreqDoneR = crate::BitReader;
#[doc = "Field `PVTM_CORE_L_FREQ_DONE` writer - pd_core_l pvtm frequency calculate done\n\nstutus"]
pub type PvtmCoreLFreqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVTM_CORE_B_FREQ_DONE` reader - pd_core_b pvtm frequency calculate done\n\nstutus"]
pub type PvtmCoreBFreqDoneR = crate::BitReader;
#[doc = "Field `PVTM_CORE_B_FREQ_DONE` writer - pd_core_b pvtm frequency calculate done\n\nstutus"]
pub type PvtmCoreBFreqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVTM_GPU_FREQ_DONE` reader - gpu pvtm frequency calculate done stutus"]
pub type PvtmGpuFreqDoneR = crate::BitReader;
#[doc = "Field `PVTM_GPU_FREQ_DONE` writer - gpu pvtm frequency calculate done stutus"]
pub type PvtmGpuFreqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVTM_DDR_FREQ_DONE` reader - ddr pvtm frequency calculate done stutus"]
pub type PvtmDdrFreqDoneR = crate::BitReader;
#[doc = "Field `PVTM_DDR_FREQ_DONE` writer - ddr pvtm frequency calculate done stutus"]
pub type PvtmDdrFreqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pd_core_l pvtm frequency calculate done\n\nstutus"]
    #[inline(always)]
    pub fn pvtm_core_l_freq_done(&self) -> PvtmCoreLFreqDoneR {
        PvtmCoreLFreqDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pd_core_b pvtm frequency calculate done\n\nstutus"]
    #[inline(always)]
    pub fn pvtm_core_b_freq_done(&self) -> PvtmCoreBFreqDoneR {
        PvtmCoreBFreqDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - gpu pvtm frequency calculate done stutus"]
    #[inline(always)]
    pub fn pvtm_gpu_freq_done(&self) -> PvtmGpuFreqDoneR {
        PvtmGpuFreqDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ddr pvtm frequency calculate done stutus"]
    #[inline(always)]
    pub fn pvtm_ddr_freq_done(&self) -> PvtmDdrFreqDoneR {
        PvtmDdrFreqDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - pd_core_l pvtm frequency calculate done\n\nstutus"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_l_freq_done(&mut self) -> PvtmCoreLFreqDoneW<GrfDllStatus0Spec> {
        PvtmCoreLFreqDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - pd_core_b pvtm frequency calculate done\n\nstutus"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_b_freq_done(&mut self) -> PvtmCoreBFreqDoneW<GrfDllStatus0Spec> {
        PvtmCoreBFreqDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - gpu pvtm frequency calculate done stutus"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_gpu_freq_done(&mut self) -> PvtmGpuFreqDoneW<GrfDllStatus0Spec> {
        PvtmGpuFreqDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - ddr pvtm frequency calculate done stutus"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_ddr_freq_done(&mut self) -> PvtmDdrFreqDoneW<GrfDllStatus0Spec> {
        PvtmDdrFreqDoneW::new(self, 3)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfDllStatus0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfDllStatus0Spec;
impl crate::RegisterSpec for GrfDllStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_dll_status0::R`](R) reader structure"]
impl crate::Readable for GrfDllStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_dll_status0::W`](W) writer structure"]
impl crate::Writable for GrfDllStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_DLL_STATUS0 to value 0"]
impl crate::Resettable for GrfDllStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
