#[doc = "Register `GRF_DLL_CON5` reader"]
pub type R = crate::R<GrfDllCon5Spec>;
#[doc = "Register `GRF_DLL_CON5` writer"]
pub type W = crate::W<GrfDllCon5Spec>;
#[doc = "Field `PVTM_CORE_B_OSC_SEL` reader - pvtm_core_b_osc_sel\\[2\\]"]
pub type PvtmCoreBOscSelR = crate::BitReader;
#[doc = "Field `PVTM_CORE_B_OSC_SEL` writer - pvtm_core_b_osc_sel\\[2\\]"]
pub type PvtmCoreBOscSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pvtm_core_b_osc_sel\\[2\\]"]
    #[inline(always)]
    pub fn pvtm_core_b_osc_sel(&self) -> PvtmCoreBOscSelR {
        PvtmCoreBOscSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - pvtm_core_b_osc_sel\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_b_osc_sel(&mut self) -> PvtmCoreBOscSelW<GrfDllCon5Spec> {
        PvtmCoreBOscSelW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfDllCon5Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_con5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_con5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfDllCon5Spec;
impl crate::RegisterSpec for GrfDllCon5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_dll_con5::R`](R) reader structure"]
impl crate::Readable for GrfDllCon5Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_dll_con5::W`](W) writer structure"]
impl crate::Writable for GrfDllCon5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_DLL_CON5 to value 0"]
impl crate::Resettable for GrfDllCon5Spec {
    const RESET_VALUE: u32 = 0;
}
