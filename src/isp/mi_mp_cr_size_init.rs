#[doc = "Register `MI_MP_CR_SIZE_INIT` reader"]
pub type R = crate::R<MiMpCrSizeInitSpec>;
#[doc = "Register `MI_MP_CR_SIZE_INIT` writer"]
pub type W = crate::W<MiMpCrSizeInitSpec>;
#[doc = "Field `mp_cr_size_init` reader - .\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect.\n\n"]
pub type MpCrSizeInitR = crate::FieldReader<u32>;
#[doc = "Field `mp_cr_size_init` writer - .\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect.\n\n"]
pub type MpCrSizeInitW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 3:27 - .\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect.\n\n"]
    #[inline(always)]
    pub fn mp_cr_size_init(&self) -> MpCrSizeInitR {
        MpCrSizeInitR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:27 - .\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn mp_cr_size_init(&mut self) -> MpCrSizeInitW<MiMpCrSizeInitSpec> {
        MpCrSizeInitW::new(self, 3)
    }
}
#[doc = "Size of main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_size_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cr_size_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpCrSizeInitSpec;
impl crate::RegisterSpec for MiMpCrSizeInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_cr_size_init::R`](R) reader structure"]
impl crate::Readable for MiMpCrSizeInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_mp_cr_size_init::W`](W) writer structure"]
impl crate::Writable for MiMpCrSizeInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_MP_CR_SIZE_INIT to value 0"]
impl crate::Resettable for MiMpCrSizeInitSpec {
    const RESET_VALUE: u32 = 0;
}
