#[doc = "Register `CRU_SOFTRST_CON9` reader"]
pub type R = crate::R<CruSoftrstCon9Spec>;
#[doc = "Register `CRU_SOFTRST_CON9` writer"]
pub type W = crate::W<CruSoftrstCon9Spec>;
#[doc = "Field `RESETN_USB2PHY0_POR_REQ` reader - resetn_usb2phy0_por request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy0PorReqR = crate::BitReader;
#[doc = "Field `RESETN_USB2PHY0_POR_REQ` writer - resetn_usb2phy0_por request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy0PorReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_USB2PHY0_UTMI_PORT0_REQ` reader - resetn_usb2phy0_utmi_port0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy0UtmiPort0ReqR = crate::BitReader;
#[doc = "Field `RESETN_USB2PHY0_UTMI_PORT0_REQ` writer - resetn_usb2phy0_utmi_port0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy0UtmiPort0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_USB2PHY0_UTMI_PORT1_REQ` reader - resetn_usb2phy0_utmi_port1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy0UtmiPort1ReqR = crate::BitReader;
#[doc = "Field `RESETN_USB2PHY0_UTMI_PORT1_REQ` writer - resetn_usb2phy0_utmi_port1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy0UtmiPort1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_USB2PHY0_EHCIPHY_REQ` reader - resetn_usb2phy0_ehciphy request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy0EhciphyReqR = crate::BitReader;
#[doc = "Field `RESETN_USB2PHY0_EHCIPHY_REQ` writer - resetn_usb2phy0_ehciphy request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy0EhciphyReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UPHY0_PIPE_L00_REQ` reader - resetn_uphy0_pipe_l00 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy0PipeL00ReqR = crate::BitReader;
#[doc = "Field `RESETN_UPHY0_PIPE_L00_REQ` writer - resetn_uphy0_pipe_l00 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy0PipeL00ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UPHY0_REQ` reader - resetn_uphy0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy0ReqR = crate::BitReader;
#[doc = "Field `RESETN_UPHY0_REQ` writer - resetn_uphy0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UPHY0_TCPDPWRUP_REQ` reader - resetn_uphy0_tcpdpwrup request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy0TcpdpwrupReqR = crate::BitReader;
#[doc = "Field `RESETN_UPHY0_TCPDPWRUP_REQ` writer - resetn_uphy0_tcpdpwrup request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy0TcpdpwrupReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_USB2PHY1_POR_REQ` reader - resetn_usb2phy1_por request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy1PorReqR = crate::BitReader;
#[doc = "Field `RESETN_USB2PHY1_POR_REQ` writer - resetn_usb2phy1_por request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy1PorReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_USB2PHY1_UTMI_PORT0_REQ` reader - resetn_usb2phy1_utmi_port0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy1UtmiPort0ReqR = crate::BitReader;
#[doc = "Field `RESETN_USB2PHY1_UTMI_PORT0_REQ` writer - resetn_usb2phy1_utmi_port0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy1UtmiPort0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_USB2PHY1_UTMI_PORT1_REQ` reader - resetn_usb2phy1_utmi_port1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy1UtmiPort1ReqR = crate::BitReader;
#[doc = "Field `RESETN_USB2PHY1_UTMI_PORT1_REQ` writer - resetn_usb2phy1_utmi_port1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy1UtmiPort1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_USB2PHY1_EHCIPHY_REQ` reader - resetn_usb2phy1_ehciphy request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy1EhciphyReqR = crate::BitReader;
#[doc = "Field `RESETN_USB2PHY1_EHCIPHY_REQ` writer - resetn_usb2phy1_ehciphy request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUsb2phy1EhciphyReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UPHY1_PIPE_L00_REQ` reader - resetn_uphy1_pipe_l00 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy1PipeL00ReqR = crate::BitReader;
#[doc = "Field `RESETN_UPHY1_PIPE_L00_REQ` writer - resetn_uphy1_pipe_l00 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy1PipeL00ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UPHY1_REQ` reader - resetn_uphy1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy1ReqR = crate::BitReader;
#[doc = "Field `RESETN_UPHY1_REQ` writer - resetn_uphy1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UPHY1_TCPDPWRUP_REQ` reader - resetn_uphy1_tcpdpwrup request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy1TcpdpwrupReqR = crate::BitReader;
#[doc = "Field `RESETN_UPHY1_TCPDPWRUP_REQ` writer - resetn_uphy1_tcpdpwrup request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUphy1TcpdpwrupReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - resetn_usb2phy0_por request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_usb2phy0_por_req(&self) -> ResetnUsb2phy0PorReqR {
        ResetnUsb2phy0PorReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - resetn_usb2phy0_utmi_port0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_usb2phy0_utmi_port0_req(&self) -> ResetnUsb2phy0UtmiPort0ReqR {
        ResetnUsb2phy0UtmiPort0ReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - resetn_usb2phy0_utmi_port1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_usb2phy0_utmi_port1_req(&self) -> ResetnUsb2phy0UtmiPort1ReqR {
        ResetnUsb2phy0UtmiPort1ReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - resetn_usb2phy0_ehciphy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_usb2phy0_ehciphy_req(&self) -> ResetnUsb2phy0EhciphyReqR {
        ResetnUsb2phy0EhciphyReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - resetn_uphy0_pipe_l00 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uphy0_pipe_l00_req(&self) -> ResetnUphy0PipeL00ReqR {
        ResetnUphy0PipeL00ReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - resetn_uphy0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uphy0_req(&self) -> ResetnUphy0ReqR {
        ResetnUphy0ReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - resetn_uphy0_tcpdpwrup request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uphy0_tcpdpwrup_req(&self) -> ResetnUphy0TcpdpwrupReqR {
        ResetnUphy0TcpdpwrupReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - resetn_usb2phy1_por request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_usb2phy1_por_req(&self) -> ResetnUsb2phy1PorReqR {
        ResetnUsb2phy1PorReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - resetn_usb2phy1_utmi_port0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_usb2phy1_utmi_port0_req(&self) -> ResetnUsb2phy1UtmiPort0ReqR {
        ResetnUsb2phy1UtmiPort0ReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - resetn_usb2phy1_utmi_port1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_usb2phy1_utmi_port1_req(&self) -> ResetnUsb2phy1UtmiPort1ReqR {
        ResetnUsb2phy1UtmiPort1ReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - resetn_usb2phy1_ehciphy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_usb2phy1_ehciphy_req(&self) -> ResetnUsb2phy1EhciphyReqR {
        ResetnUsb2phy1EhciphyReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - resetn_uphy1_pipe_l00 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uphy1_pipe_l00_req(&self) -> ResetnUphy1PipeL00ReqR {
        ResetnUphy1PipeL00ReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - resetn_uphy1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uphy1_req(&self) -> ResetnUphy1ReqR {
        ResetnUphy1ReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - resetn_uphy1_tcpdpwrup request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uphy1_tcpdpwrup_req(&self) -> ResetnUphy1TcpdpwrupReqR {
        ResetnUphy1TcpdpwrupReqR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - resetn_usb2phy0_por request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_usb2phy0_por_req(&mut self) -> ResetnUsb2phy0PorReqW<CruSoftrstCon9Spec> {
        ResetnUsb2phy0PorReqW::new(self, 0)
    }
    #[doc = "Bit 1 - resetn_usb2phy0_utmi_port0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_usb2phy0_utmi_port0_req(
        &mut self,
    ) -> ResetnUsb2phy0UtmiPort0ReqW<CruSoftrstCon9Spec> {
        ResetnUsb2phy0UtmiPort0ReqW::new(self, 1)
    }
    #[doc = "Bit 2 - resetn_usb2phy0_utmi_port1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_usb2phy0_utmi_port1_req(
        &mut self,
    ) -> ResetnUsb2phy0UtmiPort1ReqW<CruSoftrstCon9Spec> {
        ResetnUsb2phy0UtmiPort1ReqW::new(self, 2)
    }
    #[doc = "Bit 3 - resetn_usb2phy0_ehciphy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_usb2phy0_ehciphy_req(&mut self) -> ResetnUsb2phy0EhciphyReqW<CruSoftrstCon9Spec> {
        ResetnUsb2phy0EhciphyReqW::new(self, 3)
    }
    #[doc = "Bit 4 - resetn_uphy0_pipe_l00 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uphy0_pipe_l00_req(&mut self) -> ResetnUphy0PipeL00ReqW<CruSoftrstCon9Spec> {
        ResetnUphy0PipeL00ReqW::new(self, 4)
    }
    #[doc = "Bit 5 - resetn_uphy0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uphy0_req(&mut self) -> ResetnUphy0ReqW<CruSoftrstCon9Spec> {
        ResetnUphy0ReqW::new(self, 5)
    }
    #[doc = "Bit 6 - resetn_uphy0_tcpdpwrup request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uphy0_tcpdpwrup_req(&mut self) -> ResetnUphy0TcpdpwrupReqW<CruSoftrstCon9Spec> {
        ResetnUphy0TcpdpwrupReqW::new(self, 6)
    }
    #[doc = "Bit 8 - resetn_usb2phy1_por request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_usb2phy1_por_req(&mut self) -> ResetnUsb2phy1PorReqW<CruSoftrstCon9Spec> {
        ResetnUsb2phy1PorReqW::new(self, 8)
    }
    #[doc = "Bit 9 - resetn_usb2phy1_utmi_port0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_usb2phy1_utmi_port0_req(
        &mut self,
    ) -> ResetnUsb2phy1UtmiPort0ReqW<CruSoftrstCon9Spec> {
        ResetnUsb2phy1UtmiPort0ReqW::new(self, 9)
    }
    #[doc = "Bit 10 - resetn_usb2phy1_utmi_port1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_usb2phy1_utmi_port1_req(
        &mut self,
    ) -> ResetnUsb2phy1UtmiPort1ReqW<CruSoftrstCon9Spec> {
        ResetnUsb2phy1UtmiPort1ReqW::new(self, 10)
    }
    #[doc = "Bit 11 - resetn_usb2phy1_ehciphy request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_usb2phy1_ehciphy_req(&mut self) -> ResetnUsb2phy1EhciphyReqW<CruSoftrstCon9Spec> {
        ResetnUsb2phy1EhciphyReqW::new(self, 11)
    }
    #[doc = "Bit 12 - resetn_uphy1_pipe_l00 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uphy1_pipe_l00_req(&mut self) -> ResetnUphy1PipeL00ReqW<CruSoftrstCon9Spec> {
        ResetnUphy1PipeL00ReqW::new(self, 12)
    }
    #[doc = "Bit 13 - resetn_uphy1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uphy1_req(&mut self) -> ResetnUphy1ReqW<CruSoftrstCon9Spec> {
        ResetnUphy1ReqW::new(self, 13)
    }
    #[doc = "Bit 14 - resetn_uphy1_tcpdpwrup request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uphy1_tcpdpwrup_req(&mut self) -> ResetnUphy1TcpdpwrupReqW<CruSoftrstCon9Spec> {
        ResetnUphy1TcpdpwrupReqW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon9Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon9Spec;
impl crate::RegisterSpec for CruSoftrstCon9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con9::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon9Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con9::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON9 to value 0"]
impl crate::Resettable for CruSoftrstCon9Spec {
    const RESET_VALUE: u32 = 0;
}
