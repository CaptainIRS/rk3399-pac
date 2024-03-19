#[doc = "Register `A_KSVMEMCTRL` reader"]
pub type R = crate::R<AKsvmemctrlSpec>;
#[doc = "Register `A_KSVMEMCTRL` writer"]
pub type W = crate::W<AKsvmemctrlSpec>;
#[doc = "Field `KSVMEMREQUEST` reader - Request access to the KSV memory; must be de-\n\nasserted after the access is completed by the\n\nsystem."]
pub type KsvmemrequestR = crate::BitReader;
#[doc = "Field `KSVMEMREQUEST` writer - Request access to the KSV memory; must be de-\n\nasserted after the access is completed by the\n\nsystem."]
pub type KsvmemrequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KSVMEMACCESS` reader - Notification that the KSV Attr as been guaranteed."]
pub type KsvmemaccessR = crate::BitReader;
#[doc = "Field `KSVCTRLUPD` reader - Set to inform that the KSV list in memory has been\n\nanalyzed and the response to the Message Digest\n\nhas been updated if on configurations on software\n\nSHA-1 calculation."]
pub type KsvctrlupdR = crate::BitReader;
#[doc = "Field `KSVCTRLUPD` writer - Set to inform that the KSV list in memory has been\n\nanalyzed and the response to the Message Digest\n\nhas been updated if on configurations on software\n\nSHA-1 calculation."]
pub type KsvctrlupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA1FAIL` reader - Notification whether the KSV list message digest is\n\ncorrect."]
pub type Sha1failR = crate::BitReader;
#[doc = "Field `SHA1FAIL` writer - Notification whether the KSV list message digest is\n\ncorrect."]
pub type Sha1failW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KSVSHA1STATUS` reader - Notification whether the KSV list message digest is\n\ncorrect from the controller: 1'b1 if digest message\n\nverification failed 1'b0 if digest message verification\n\nsucceeded"]
pub type Ksvsha1statusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Request access to the KSV memory; must be de-\n\nasserted after the access is completed by the\n\nsystem."]
    #[inline(always)]
    pub fn ksvmemrequest(&self) -> KsvmemrequestR {
        KsvmemrequestR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Notification that the KSV Attr as been guaranteed."]
    #[inline(always)]
    pub fn ksvmemaccess(&self) -> KsvmemaccessR {
        KsvmemaccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to inform that the KSV list in memory has been\n\nanalyzed and the response to the Message Digest\n\nhas been updated if on configurations on software\n\nSHA-1 calculation."]
    #[inline(always)]
    pub fn ksvctrlupd(&self) -> KsvctrlupdR {
        KsvctrlupdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Notification whether the KSV list message digest is\n\ncorrect."]
    #[inline(always)]
    pub fn sha1fail(&self) -> Sha1failR {
        Sha1failR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Notification whether the KSV list message digest is\n\ncorrect from the controller: 1'b1 if digest message\n\nverification failed 1'b0 if digest message verification\n\nsucceeded"]
    #[inline(always)]
    pub fn ksvsha1status(&self) -> Ksvsha1statusR {
        Ksvsha1statusR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request access to the KSV memory; must be de-\n\nasserted after the access is completed by the\n\nsystem."]
    #[inline(always)]
    #[must_use]
    pub fn ksvmemrequest(&mut self) -> KsvmemrequestW<AKsvmemctrlSpec> {
        KsvmemrequestW::new(self, 0)
    }
    #[doc = "Bit 2 - Set to inform that the KSV list in memory has been\n\nanalyzed and the response to the Message Digest\n\nhas been updated if on configurations on software\n\nSHA-1 calculation."]
    #[inline(always)]
    #[must_use]
    pub fn ksvctrlupd(&mut self) -> KsvctrlupdW<AKsvmemctrlSpec> {
        KsvctrlupdW::new(self, 2)
    }
    #[doc = "Bit 3 - Notification whether the KSV list message digest is\n\ncorrect."]
    #[inline(always)]
    #[must_use]
    pub fn sha1fail(&mut self) -> Sha1failW<AKsvmemctrlSpec> {
        Sha1failW::new(self, 3)
    }
}
#[doc = "HDCP KSV Memory Control Register\n\nThe KSVCTRLupd bit is a notification flag. This flag changes polarity whenever the register\n\nis written. This flag acts as a trigger to other blocks that processes this data. Upon reset\n\nthe flag returns to low default value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_ksvmemctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_ksvmemctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AKsvmemctrlSpec;
impl crate::RegisterSpec for AKsvmemctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_ksvmemctrl::R`](R) reader structure"]
impl crate::Readable for AKsvmemctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`a_ksvmemctrl::W`](W) writer structure"]
impl crate::Writable for AKsvmemctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets A_KSVMEMCTRL to value 0"]
impl crate::Resettable for AKsvmemctrlSpec {
    const RESET_VALUE: u8 = 0;
}
