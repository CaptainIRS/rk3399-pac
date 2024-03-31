#[doc = "Register `AXI_BUS_MODE` reader"]
pub type R = crate::R<AxiBusModeSpec>;
#[doc = "Register `AXI_BUS_MODE` writer"]
pub type W = crate::W<AxiBusModeSpec>;
#[doc = "Field `UNDEF` reader - AXI Undefined Burst Length\n\nThis bit is read-only bit and indicates the complement (invert)\n\nvalue of FB bit in register GMAC_BUS_MODE\\[16\\].\n\nWhen this bit is set to 1, it is allowed to perform any burst length\n\nequal to or below the maximum allowed burst length as\n\nprogrammed in bits\\[7:1\\];\n\nWhen this bit is set to 0, it is allowed to perform only fixed burst\n\nlengths as indicated by BLEN256/128/64/32/16/8/4, or a burst\n\nlength of 1."]
pub type UndefR = crate::BitReader;
#[doc = "Field `BLEN4` reader - AXI Burst Length 4\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 4."]
pub type Blen4R = crate::BitReader;
#[doc = "Field `BLEN4` writer - AXI Burst Length 4\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 4."]
pub type Blen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN8` reader - AXI Burst Length 8\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 8."]
pub type Blen8R = crate::BitReader;
#[doc = "Field `BLEN8` writer - AXI Burst Length 8\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 8."]
pub type Blen8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN16` reader - AXI Burst Length 16\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 16."]
pub type Blen16R = crate::BitReader;
#[doc = "Field `BLEN16` writer - AXI Burst Length 16\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 16."]
pub type Blen16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_AAL` reader - Address-Aligned Beats\n\nThis bit is read-only bit and reflects the AAL bit (register\n\nGMAC_BUS_MODE\\[25\\]).\n\nWhen this bit set to 1, it performs address-aligned burst transfers\n\non both read and write channels."]
pub type AxiAalR = crate::BitReader;
#[doc = "Field `RD_OSR_LMT` reader - AXI Maximum Read Out Standing Request Limit\n\nThis value limits the maximum outstanding request on the AXI\n\nread interface.\n\nMaximum outstanding requests = RD_OSR_LMT+1"]
pub type RdOsrLmtR = crate::FieldReader;
#[doc = "Field `RD_OSR_LMT` writer - AXI Maximum Read Out Standing Request Limit\n\nThis value limits the maximum outstanding request on the AXI\n\nread interface.\n\nMaximum outstanding requests = RD_OSR_LMT+1"]
pub type RdOsrLmtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WR_OSR_LMT` reader - AXI Maximum Write Out Standing Request Limit\n\nThis value limits the maximum outstanding request on the AXI\n\nwrite interface.\n\nMaximum outstanding requests = WR_OSR_LMT+1"]
pub type WrOsrLmtR = crate::FieldReader;
#[doc = "Field `WR_OSR_LMT` writer - AXI Maximum Write Out Standing Request Limit\n\nThis value limits the maximum outstanding request on the AXI\n\nwrite interface.\n\nMaximum outstanding requests = WR_OSR_LMT+1"]
pub type WrOsrLmtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UNLCK_ON_MGK_RWK` reader - Unlock on Magic Packet or Remote Wake Up\n\nWhen set to 1, enables it to request coming out of Low Power\n\nmode only when Magic Packet or Remote Wake Up Packet is\n\nreceived.\n\nWhen set to 0, enables it requests to come out of Low Power\n\nmode when any frame is received."]
pub type UnlckOnMgkRwkR = crate::BitReader;
#[doc = "Field `UNLCK_ON_MGK_RWK` writer - Unlock on Magic Packet or Remote Wake Up\n\nWhen set to 1, enables it to request coming out of Low Power\n\nmode only when Magic Packet or Remote Wake Up Packet is\n\nreceived.\n\nWhen set to 0, enables it requests to come out of Low Power\n\nmode when any frame is received."]
pub type UnlckOnMgkRwkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_LPI` reader - Enable LPI (Low Power Interface)\n\nWhen set to 1, enable the LPI (Low Power Interface) supported\n\nby the GMAC and accepts the LPI request from the AXI System\n\nClock controller.\n\nWhen set to 0, disables the Low Power Mode and always denies\n\nthe LPI request from the AXI System Clock controller."]
pub type EnLpiR = crate::BitReader;
#[doc = "Field `EN_LPI` writer - Enable LPI (Low Power Interface)\n\nWhen set to 1, enable the LPI (Low Power Interface) supported\n\nby the GMAC and accepts the LPI request from the AXI System\n\nClock controller.\n\nWhen set to 0, disables the Low Power Mode and always denies\n\nthe LPI request from the AXI System Clock controller."]
pub type EnLpiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AXI Undefined Burst Length\n\nThis bit is read-only bit and indicates the complement (invert)\n\nvalue of FB bit in register GMAC_BUS_MODE\\[16\\].\n\nWhen this bit is set to 1, it is allowed to perform any burst length\n\nequal to or below the maximum allowed burst length as\n\nprogrammed in bits\\[7:1\\];\n\nWhen this bit is set to 0, it is allowed to perform only fixed burst\n\nlengths as indicated by BLEN256/128/64/32/16/8/4, or a burst\n\nlength of 1."]
    #[inline(always)]
    pub fn undef(&self) -> UndefR {
        UndefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AXI Burst Length 4\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 4."]
    #[inline(always)]
    pub fn blen4(&self) -> Blen4R {
        Blen4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AXI Burst Length 8\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 8."]
    #[inline(always)]
    pub fn blen8(&self) -> Blen8R {
        Blen8R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI Burst Length 16\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 16."]
    #[inline(always)]
    pub fn blen16(&self) -> Blen16R {
        Blen16R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats\n\nThis bit is read-only bit and reflects the AAL bit (register\n\nGMAC_BUS_MODE\\[25\\]).\n\nWhen this bit set to 1, it performs address-aligned burst transfers\n\non both read and write channels."]
    #[inline(always)]
    pub fn axi_aal(&self) -> AxiAalR {
        AxiAalR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - AXI Maximum Read Out Standing Request Limit\n\nThis value limits the maximum outstanding request on the AXI\n\nread interface.\n\nMaximum outstanding requests = RD_OSR_LMT+1"]
    #[inline(always)]
    pub fn rd_osr_lmt(&self) -> RdOsrLmtR {
        RdOsrLmtR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - AXI Maximum Write Out Standing Request Limit\n\nThis value limits the maximum outstanding request on the AXI\n\nwrite interface.\n\nMaximum outstanding requests = WR_OSR_LMT+1"]
    #[inline(always)]
    pub fn wr_osr_lmt(&self) -> WrOsrLmtR {
        WrOsrLmtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 30 - Unlock on Magic Packet or Remote Wake Up\n\nWhen set to 1, enables it to request coming out of Low Power\n\nmode only when Magic Packet or Remote Wake Up Packet is\n\nreceived.\n\nWhen set to 0, enables it requests to come out of Low Power\n\nmode when any frame is received."]
    #[inline(always)]
    pub fn unlck_on_mgk_rwk(&self) -> UnlckOnMgkRwkR {
        UnlckOnMgkRwkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable LPI (Low Power Interface)\n\nWhen set to 1, enable the LPI (Low Power Interface) supported\n\nby the GMAC and accepts the LPI request from the AXI System\n\nClock controller.\n\nWhen set to 0, disables the Low Power Mode and always denies\n\nthe LPI request from the AXI System Clock controller."]
    #[inline(always)]
    pub fn en_lpi(&self) -> EnLpiR {
        EnLpiR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - AXI Burst Length 4\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 4."]
    #[inline(always)]
    #[must_use]
    pub fn blen4(&mut self) -> Blen4W<AxiBusModeSpec> {
        Blen4W::new(self, 1)
    }
    #[doc = "Bit 2 - AXI Burst Length 8\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 8."]
    #[inline(always)]
    #[must_use]
    pub fn blen8(&mut self) -> Blen8W<AxiBusModeSpec> {
        Blen8W::new(self, 2)
    }
    #[doc = "Bit 3 - AXI Burst Length 16\n\nWhen this bit is set to 1, or when UNDEF is set to 1, it is allowed\n\nto select a burst length of 16."]
    #[inline(always)]
    #[must_use]
    pub fn blen16(&mut self) -> Blen16W<AxiBusModeSpec> {
        Blen16W::new(self, 3)
    }
    #[doc = "Bits 16:17 - AXI Maximum Read Out Standing Request Limit\n\nThis value limits the maximum outstanding request on the AXI\n\nread interface.\n\nMaximum outstanding requests = RD_OSR_LMT+1"]
    #[inline(always)]
    #[must_use]
    pub fn rd_osr_lmt(&mut self) -> RdOsrLmtW<AxiBusModeSpec> {
        RdOsrLmtW::new(self, 16)
    }
    #[doc = "Bits 20:21 - AXI Maximum Write Out Standing Request Limit\n\nThis value limits the maximum outstanding request on the AXI\n\nwrite interface.\n\nMaximum outstanding requests = WR_OSR_LMT+1"]
    #[inline(always)]
    #[must_use]
    pub fn wr_osr_lmt(&mut self) -> WrOsrLmtW<AxiBusModeSpec> {
        WrOsrLmtW::new(self, 20)
    }
    #[doc = "Bit 30 - Unlock on Magic Packet or Remote Wake Up\n\nWhen set to 1, enables it to request coming out of Low Power\n\nmode only when Magic Packet or Remote Wake Up Packet is\n\nreceived.\n\nWhen set to 0, enables it requests to come out of Low Power\n\nmode when any frame is received."]
    #[inline(always)]
    #[must_use]
    pub fn unlck_on_mgk_rwk(&mut self) -> UnlckOnMgkRwkW<AxiBusModeSpec> {
        UnlckOnMgkRwkW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable LPI (Low Power Interface)\n\nWhen set to 1, enable the LPI (Low Power Interface) supported\n\nby the GMAC and accepts the LPI request from the AXI System\n\nClock controller.\n\nWhen set to 0, disables the Low Power Mode and always denies\n\nthe LPI request from the AXI System Clock controller."]
    #[inline(always)]
    #[must_use]
    pub fn en_lpi(&mut self) -> EnLpiW<AxiBusModeSpec> {
        EnLpiW::new(self, 31)
    }
}
#[doc = "AXI Bus Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_bus_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_bus_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiBusModeSpec;
impl crate::RegisterSpec for AxiBusModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_bus_mode::R`](R) reader structure"]
impl crate::Readable for AxiBusModeSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_bus_mode::W`](W) writer structure"]
impl crate::Writable for AxiBusModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AXI_BUS_MODE to value 0x0011_0001"]
impl crate::Resettable for AxiBusModeSpec {
    const RESET_VALUE: u32 = 0x0011_0001;
}
