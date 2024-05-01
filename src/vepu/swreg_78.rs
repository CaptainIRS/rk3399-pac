#[doc = "Register `SWREG_78` reader"]
pub type R = crate::R<Swreg78Spec>;
#[doc = "Register `SWREG_78` writer"]
pub type W = crate::W<Swreg78Spec>;
#[doc = "Field `OUTPUT_CTRL_ST_ADR` reader - output control start address\n\noutput control start address"]
pub type OutputCtrlStAdrR = crate::FieldReader<u32>;
#[doc = "Field `OUTPUT_CTRL_ST_ADR` writer - output control start address\n\noutput control start address"]
pub type OutputCtrlStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - output control start address\n\noutput control start address"]
    #[inline(always)]
    pub fn output_ctrl_st_adr(&self) -> OutputCtrlStAdrR {
        OutputCtrlStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - output control start address\n\noutput control start address"]
    #[inline(always)]
    #[must_use]
    pub fn output_ctrl_st_adr(&mut self) -> OutputCtrlStAdrW<Swreg78Spec> {
        OutputCtrlStAdrW::new(self, 0)
    }
}
#[doc = "output control start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_78::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_78::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg78Spec;
impl crate::RegisterSpec for Swreg78Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_78::R`](R) reader structure"]
impl crate::Readable for Swreg78Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_78::W`](W) writer structure"]
impl crate::Writable for Swreg78Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_78 to value 0"]
impl crate::Resettable for Swreg78Spec {
    const RESET_VALUE: u32 = 0;
}
