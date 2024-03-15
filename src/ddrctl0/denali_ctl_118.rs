#[doc = "Register `DENALI_CTL_118` reader"]
pub type R = crate::R<DenaliCtl118Spec>;
#[doc = "Register `DENALI_CTL_118` writer"]
pub type W = crate::W<DenaliCtl118Spec>;
#[doc = "Field `MRW_STATUS` reader - Write memory mode register status. Bit (0) set indicates a WRITE_MODEREG parameter programming error. Bit (1) set indicates a PASR error. Bit (2) is Reserved. Bit (3) set indicates a self refresh or deep power down error. Bit (4) set indicates that a write to MR3 or MR11 was attempted (WRITE_MODEREG bit (25) was asserted with bit (17) set, or bit (23) was asserted with bits (7:0) defining MR3 or MR11) during tZQCAL after a ZQ calibration start command. READ-ONLY"]
pub type MrwStatusR = crate::FieldReader;
#[doc = "Field `READ_MODEREG` reader - Read the specified memory mode register from specified chip when start bit set. Bits (7:0) define the memory mode register and bits (15:8) define the chip select. Set bit (16) to 1 to trigger."]
pub type ReadModeregR = crate::FieldReader<u32>;
#[doc = "Field `READ_MODEREG` writer - Read the specified memory mode register from specified chip when start bit set. Bits (7:0) define the memory mode register and bits (15:8) define the chip select. Set bit (16) to 1 to trigger."]
pub type ReadModeregW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:7 - Write memory mode register status. Bit (0) set indicates a WRITE_MODEREG parameter programming error. Bit (1) set indicates a PASR error. Bit (2) is Reserved. Bit (3) set indicates a self refresh or deep power down error. Bit (4) set indicates that a write to MR3 or MR11 was attempted (WRITE_MODEREG bit (25) was asserted with bit (17) set, or bit (23) was asserted with bits (7:0) defining MR3 or MR11) during tZQCAL after a ZQ calibration start command. READ-ONLY"]
    #[inline(always)]
    pub fn mrw_status(&self) -> MrwStatusR {
        MrwStatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:24 - Read the specified memory mode register from specified chip when start bit set. Bits (7:0) define the memory mode register and bits (15:8) define the chip select. Set bit (16) to 1 to trigger."]
    #[inline(always)]
    pub fn read_modereg(&self) -> ReadModeregR {
        ReadModeregR::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 8:24 - Read the specified memory mode register from specified chip when start bit set. Bits (7:0) define the memory mode register and bits (15:8) define the chip select. Set bit (16) to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn read_modereg(&mut self) -> ReadModeregW<DenaliCtl118Spec> {
        ReadModeregW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_118::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_118::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl118Spec;
impl crate::RegisterSpec for DenaliCtl118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_118::R`](R) reader structure"]
impl crate::Readable for DenaliCtl118Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_118::W`](W) writer structure"]
impl crate::Writable for DenaliCtl118Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_118 to value 0"]
impl crate::Resettable for DenaliCtl118Spec {
    const RESET_VALUE: u32 = 0;
}
