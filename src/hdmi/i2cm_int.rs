#[doc = "Register `I2CM_INT` reader"]
pub type R = crate::R<I2cmIntSpec>;
#[doc = "Register `I2CM_INT` writer"]
pub type W = crate::W<I2cmIntSpec>;
#[doc = "Field `DONE_MASK` reader - Done interrupt mask signal."]
pub type DoneMaskR = crate::BitReader;
#[doc = "Field `DONE_MASK` writer - Done interrupt mask signal."]
pub type DoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REQ_MASK` reader - Read request interruption mask signal."]
pub type ReadReqMaskR = crate::BitReader;
#[doc = "Field `READ_REQ_MASK` writer - Read request interruption mask signal."]
pub type ReadReqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Done interrupt mask signal."]
    #[inline(always)]
    pub fn done_mask(&self) -> DoneMaskR {
        DoneMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Read request interruption mask signal."]
    #[inline(always)]
    pub fn read_req_mask(&self) -> ReadReqMaskR {
        ReadReqMaskR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Done interrupt mask signal."]
    #[inline(always)]
    #[must_use]
    pub fn done_mask(&mut self) -> DoneMaskW<I2cmIntSpec> {
        DoneMaskW::new(self, 2)
    }
    #[doc = "Bit 6 - Read request interruption mask signal."]
    #[inline(always)]
    #[must_use]
    pub fn read_req_mask(&mut self) -> ReadReqMaskW<I2cmIntSpec> {
        ReadReqMaskW::new(self, 6)
    }
}
#[doc = "Reserved for future use.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmIntSpec;
impl crate::RegisterSpec for I2cmIntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_int::R`](R) reader structure"]
impl crate::Readable for I2cmIntSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_int::W`](W) writer structure"]
impl crate::Writable for I2cmIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_INT to value 0x40"]
impl crate::Resettable for I2cmIntSpec {
    const RESET_VALUE: u8 = 0x40;
}
