#[doc = "Register `PHY_I2CM_INT` reader"]
pub type R = crate::R<PhyI2cmIntSpec>;
#[doc = "Register `PHY_I2CM_INT` writer"]
pub type W = crate::W<PhyI2cmIntSpec>;
#[doc = "Field `DONE_STATUS` reader - Operation done status bit. Marks the end of a read or write operation."]
pub type DoneStatusR = crate::BitReader;
#[doc = "Field `DONE_INTERRUPT` reader - Operation done interrupt bit. Only lasts for 1 SFR clock cycle and is auto cleared after it. {done_interrupt = (done_mask==0b) &amp;&amp; (done_status==done_pol)}"]
pub type DoneInterruptR = crate::BitReader;
#[doc = "Field `DONE_MASK` reader - Done interrupt mask signal"]
pub type DoneMaskR = crate::BitReader;
#[doc = "Field `DONE_MASK` writer - Done interrupt mask signal"]
pub type DoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE_POL` reader - Done interrupt polarity configuration"]
pub type DonePolR = crate::BitReader;
#[doc = "Field `DONE_POL` writer - Done interrupt polarity configuration"]
pub type DonePolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operation done status bit. Marks the end of a read or write operation."]
    #[inline(always)]
    pub fn done_status(&self) -> DoneStatusR {
        DoneStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation done interrupt bit. Only lasts for 1 SFR clock cycle and is auto cleared after it. {done_interrupt = (done_mask==0b) &amp;&amp; (done_status==done_pol)}"]
    #[inline(always)]
    pub fn done_interrupt(&self) -> DoneInterruptR {
        DoneInterruptR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Done interrupt mask signal"]
    #[inline(always)]
    pub fn done_mask(&self) -> DoneMaskR {
        DoneMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Done interrupt polarity configuration"]
    #[inline(always)]
    pub fn done_pol(&self) -> DonePolR {
        DonePolR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Done interrupt mask signal"]
    #[inline(always)]
    #[must_use]
    pub fn done_mask(&mut self) -> DoneMaskW<PhyI2cmIntSpec> {
        DoneMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Done interrupt polarity configuration"]
    #[inline(always)]
    #[must_use]
    pub fn done_pol(&mut self) -> DonePolW<PhyI2cmIntSpec> {
        DonePolW::new(self, 3)
    }
}
#[doc = "Operation done status bit. Marks the end of a read or write operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmIntSpec;
impl crate::RegisterSpec for PhyI2cmIntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_int::R`](R) reader structure"]
impl crate::Readable for PhyI2cmIntSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_i2cm_int::W`](W) writer structure"]
impl crate::Writable for PhyI2cmIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_I2CM_INT to value 0x08"]
impl crate::Resettable for PhyI2cmIntSpec {
    const RESET_VALUE: u8 = 0x08;
}
