#[doc = "Register `PCIE_CLIENT_MSG_CODE0` reader"]
pub type R = crate::R<PcieClientMsgCode0Spec>;
#[doc = "Register `PCIE_CLIENT_MSG_CODE0` writer"]
pub type W = crate::W<PcieClientMsgCode0Spec>;
#[doc = "Field `MTPAT1` reader - Match pattern 1 Pattern 1"]
pub type Mtpat1R = crate::FieldReader;
#[doc = "Field `MTPAT1` writer - Match pattern 1 Pattern 1"]
pub type Mtpat1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MTPAT2` reader - Match pattern 2 Pattern2"]
pub type Mtpat2R = crate::FieldReader;
#[doc = "Field `MTPAT2` writer - Match pattern 2 Pattern2"]
pub type Mtpat2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MTPAT3` reader - Match pattern 3 Pattern3"]
pub type Mtpat3R = crate::FieldReader;
#[doc = "Field `MTPAT3` writer - Match pattern 3 Pattern3"]
pub type Mtpat3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MTPAT4` reader - Match pattern 4 Pattern4"]
pub type Mtpat4R = crate::FieldReader;
#[doc = "Field `MTPAT4` writer - Match pattern 4 Pattern4"]
pub type Mtpat4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Match pattern 1 Pattern 1"]
    #[inline(always)]
    pub fn mtpat1(&self) -> Mtpat1R {
        Mtpat1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Match pattern 2 Pattern2"]
    #[inline(always)]
    pub fn mtpat2(&self) -> Mtpat2R {
        Mtpat2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Match pattern 3 Pattern3"]
    #[inline(always)]
    pub fn mtpat3(&self) -> Mtpat3R {
        Mtpat3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Match pattern 4 Pattern4"]
    #[inline(always)]
    pub fn mtpat4(&self) -> Mtpat4R {
        Mtpat4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Match pattern 1 Pattern 1"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat1(&mut self) -> Mtpat1W<PcieClientMsgCode0Spec> {
        Mtpat1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Match pattern 2 Pattern2"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat2(&mut self) -> Mtpat2W<PcieClientMsgCode0Spec> {
        Mtpat2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Match pattern 3 Pattern3"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat3(&mut self) -> Mtpat3W<PcieClientMsgCode0Spec> {
        Mtpat3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Match pattern 4 Pattern4"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat4(&mut self) -> Mtpat4W<PcieClientMsgCode0Spec> {
        Mtpat4W::new(self, 24)
    }
}
#[doc = "Message code 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_code0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_msg_code0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientMsgCode0Spec;
impl crate::RegisterSpec for PcieClientMsgCode0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_msg_code0::R`](R) reader structure"]
impl crate::Readable for PcieClientMsgCode0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_client_msg_code0::W`](W) writer structure"]
impl crate::Writable for PcieClientMsgCode0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_CLIENT_MSG_CODE0 to value 0"]
impl crate::Resettable for PcieClientMsgCode0Spec {
    const RESET_VALUE: u32 = 0;
}
