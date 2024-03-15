#[doc = "Register `I2CM_CTLINT` reader"]
pub type R = crate::R<I2cmCtlintSpec>;
#[doc = "Register `I2CM_CTLINT` writer"]
pub type W = crate::W<I2cmCtlintSpec>;
#[doc = "Field `ARBITRATION_MASK` reader - Arbitration error interrupt mask signal."]
pub type ArbitrationMaskR = crate::BitReader;
#[doc = "Field `ARBITRATION_MASK` writer - Arbitration error interrupt mask signal."]
pub type ArbitrationMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK_MASK` reader - Not acknowledge error interrupt mask signal."]
pub type NackMaskR = crate::BitReader;
#[doc = "Field `NACK_MASK` writer - Not acknowledge error interrupt mask signal."]
pub type NackMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Arbitration error interrupt mask signal."]
    #[inline(always)]
    pub fn arbitration_mask(&self) -> ArbitrationMaskR {
        ArbitrationMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Not acknowledge error interrupt mask signal."]
    #[inline(always)]
    pub fn nack_mask(&self) -> NackMaskR {
        NackMaskR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Arbitration error interrupt mask signal."]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_mask(&mut self) -> ArbitrationMaskW<I2cmCtlintSpec> {
        ArbitrationMaskW::new(self, 2)
    }
    #[doc = "Bit 6 - Not acknowledge error interrupt mask signal."]
    #[inline(always)]
    #[must_use]
    pub fn nack_mask(&mut self) -> NackMaskW<I2cmCtlintSpec> {
        NackMaskW::new(self, 6)
    }
}
#[doc = "Arbitration error interrupt mask signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_ctlint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_ctlint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmCtlintSpec;
impl crate::RegisterSpec for I2cmCtlintSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_ctlint::R`](R) reader structure"]
impl crate::Readable for I2cmCtlintSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_ctlint::W`](W) writer structure"]
impl crate::Writable for I2cmCtlintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_CTLINT to value 0"]
impl crate::Resettable for I2cmCtlintSpec {
    const RESET_VALUE: u8 = 0;
}
