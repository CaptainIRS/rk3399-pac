#[doc = "Register `PCIE_CLIENT_MSG_CODE1` reader"]
pub type R = crate::R<PcieClientMsgCode1Spec>;
#[doc = "Register `PCIE_CLIENT_MSG_CODE1` writer"]
pub type W = crate::W<PcieClientMsgCode1Spec>;
#[doc = "Field `MTPAT5` reader - Match pattern 5 Pattern5"]
pub type Mtpat5R = crate::FieldReader;
#[doc = "Field `MTPAT5` writer - Match pattern 5 Pattern5"]
pub type Mtpat5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MTPAT6` reader - Match pattern 6 Pattern6"]
pub type Mtpat6R = crate::FieldReader;
#[doc = "Field `MTPAT6` writer - Match pattern 6 Pattern6"]
pub type Mtpat6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MTPAT7` reader - Match pattern 7 Pattern7"]
pub type Mtpat7R = crate::FieldReader;
#[doc = "Field `MTPAT7` writer - Match pattern 7 Pattern7"]
pub type Mtpat7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MTPAT8` reader - Match pattern 8 Pattern8"]
pub type Mtpat8R = crate::FieldReader;
#[doc = "Field `MTPAT8` writer - Match pattern 8 Pattern8"]
pub type Mtpat8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Match pattern 5 Pattern5"]
    #[inline(always)]
    pub fn mtpat5(&self) -> Mtpat5R {
        Mtpat5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Match pattern 6 Pattern6"]
    #[inline(always)]
    pub fn mtpat6(&self) -> Mtpat6R {
        Mtpat6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Match pattern 7 Pattern7"]
    #[inline(always)]
    pub fn mtpat7(&self) -> Mtpat7R {
        Mtpat7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Match pattern 8 Pattern8"]
    #[inline(always)]
    pub fn mtpat8(&self) -> Mtpat8R {
        Mtpat8R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Match pattern 5 Pattern5"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat5(&mut self) -> Mtpat5W<PcieClientMsgCode1Spec> {
        Mtpat5W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Match pattern 6 Pattern6"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat6(&mut self) -> Mtpat6W<PcieClientMsgCode1Spec> {
        Mtpat6W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Match pattern 7 Pattern7"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat7(&mut self) -> Mtpat7W<PcieClientMsgCode1Spec> {
        Mtpat7W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Match pattern 8 Pattern8"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat8(&mut self) -> Mtpat8W<PcieClientMsgCode1Spec> {
        Mtpat8W::new(self, 24)
    }
}
#[doc = "Message code 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_code1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_msg_code1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientMsgCode1Spec;
impl crate::RegisterSpec for PcieClientMsgCode1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_msg_code1::R`](R) reader structure"]
impl crate::Readable for PcieClientMsgCode1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_client_msg_code1::W`](W) writer structure"]
impl crate::Writable for PcieClientMsgCode1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_CLIENT_MSG_CODE1 to value 0"]
impl crate::Resettable for PcieClientMsgCode1Spec {
    const RESET_VALUE: u32 = 0;
}
