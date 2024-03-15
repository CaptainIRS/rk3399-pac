#[doc = "Register `A_OESSWCFG` reader"]
pub type R = crate::R<AOesswcfgSpec>;
#[doc = "Register `A_OESSWCFG` writer"]
pub type W = crate::W<AOesswcfgSpec>;
#[doc = "Field `A_OESSWCFG` reader - HDCP OESS WOO Configuration Register"]
pub type AOesswcfgR = crate::FieldReader;
#[doc = "Field `A_OESSWCFG` writer - HDCP OESS WOO Configuration Register"]
pub type AOesswcfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HDCP OESS WOO Configuration Register"]
    #[inline(always)]
    pub fn a_oesswcfg(&self) -> AOesswcfgR {
        AOesswcfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDCP OESS WOO Configuration Register"]
    #[inline(always)]
    #[must_use]
    pub fn a_oesswcfg(&mut self) -> AOesswcfgW<AOesswcfgSpec> {
        AOesswcfgW::new(self, 0)
    }
}
#[doc = "HDCP OESS WOO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_oesswcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_oesswcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AOesswcfgSpec;
impl crate::RegisterSpec for AOesswcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_oesswcfg::R`](R) reader structure"]
impl crate::Readable for AOesswcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`a_oesswcfg::W`](W) writer structure"]
impl crate::Writable for AOesswcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets A_OESSWCFG to value 0x80"]
impl crate::Resettable for AOesswcfgSpec {
    const RESET_VALUE: u8 = 0x80;
}
