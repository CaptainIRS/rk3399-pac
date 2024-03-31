#[doc = "Register `DENALI_CTL_187` reader"]
pub type R = crate::R<DenaliCtl187Spec>;
#[doc = "Register `DENALI_CTL_187` writer"]
pub type W = crate::W<DenaliCtl187Spec>;
#[doc = "Field `ZQ_REQ` writer - User request to initiate a ZQ calibration. Program to 0x1 for ZQ Short (ZQCS), program to 0x2 for ZQ Long (ZQCL), program to 0x3 for ZQ Start, program to 0x4 for ZQ Initialization (ZQINIT), program to 0x5 for ZQ Latch, or program to 0x8 for ZQ Reset. Clearing to 0x0 will not trigger any ZQ command. This parameter should only be written when the ZQ_REQ_PENDING parameter is cleared to 0."]
pub type ZqReqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ZQ_REQ_PENDING` reader - Indicates that a ZQ command is currently in progress or waiting to run. Value of 1 indicates command in progress or waiting to run. When this is asserted, no writes to ZQ_REQ should occur."]
pub type ZqReqPendingR = crate::BitReader;
#[doc = "Field `ZQRESET_F0` reader - Number of cycles needed for a ZQRESET command for frequency copy 0."]
pub type ZqresetF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQRESET_F0` writer - Number of cycles needed for a ZQRESET command for frequency copy 0."]
pub type ZqresetF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 8 - Indicates that a ZQ command is currently in progress or waiting to run. Value of 1 indicates command in progress or waiting to run. When this is asserted, no writes to ZQ_REQ should occur."]
    #[inline(always)]
    pub fn zq_req_pending(&self) -> ZqReqPendingR {
        ZqReqPendingR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Number of cycles needed for a ZQRESET command for frequency copy 0."]
    #[inline(always)]
    pub fn zqreset_f0(&self) -> ZqresetF0R {
        ZqresetF0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - User request to initiate a ZQ calibration. Program to 0x1 for ZQ Short (ZQCS), program to 0x2 for ZQ Long (ZQCL), program to 0x3 for ZQ Start, program to 0x4 for ZQ Initialization (ZQINIT), program to 0x5 for ZQ Latch, or program to 0x8 for ZQ Reset. Clearing to 0x0 will not trigger any ZQ command. This parameter should only be written when the ZQ_REQ_PENDING parameter is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn zq_req(&mut self) -> ZqReqW<DenaliCtl187Spec> {
        ZqReqW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Number of cycles needed for a ZQRESET command for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn zqreset_f0(&mut self) -> ZqresetF0W<DenaliCtl187Spec> {
        ZqresetF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_187::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_187::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl187Spec;
impl crate::RegisterSpec for DenaliCtl187Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_187::R`](R) reader structure"]
impl crate::Readable for DenaliCtl187Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_187::W`](W) writer structure"]
impl crate::Writable for DenaliCtl187Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_187 to value 0"]
impl crate::Resettable for DenaliCtl187Spec {
    const RESET_VALUE: u32 = 0;
}
