#[doc = "Register `USB3_GSBUSCFG1` reader"]
pub type R = crate::R<Usb3Gsbuscfg1Spec>;
#[doc = "Register `USB3_GSBUSCFG1` writer"]
pub type W = crate::W<Usb3Gsbuscfg1Spec>;
#[doc = "AXI Pipelined Transfers Burst Request Limit\n\nThe field controls the number of outstanding pipelined transfer\n\nrequests the AXI master pushes to the AXI slave. When the AXI\n\nmaster reaches this limit, it does not make any more requests on\n\nthe AXI ARADDR and AWADDR buses until the associated data\n\nphases complete.\n\nThis field is encoded as follows:\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pipetranslimit {
    #[doc = "0: 1 request"]
    D0 = 0,
    #[doc = "1: 2 requests"]
    D1 = 1,
    #[doc = "2: 3 requests"]
    D2 = 2,
    #[doc = "3: 4 requests ... F: 16 requests"]
    D3 = 3,
}
impl From<Pipetranslimit> for u8 {
    #[inline(always)]
    fn from(variant: Pipetranslimit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pipetranslimit {
    type Ux = u8;
}
#[doc = "Field `PIPETRANSLIMIT` reader - AXI Pipelined Transfers Burst Request Limit\n\nThe field controls the number of outstanding pipelined transfer\n\nrequests the AXI master pushes to the AXI slave. When the AXI\n\nmaster reaches this limit, it does not make any more requests on\n\nthe AXI ARADDR and AWADDR buses until the associated data\n\nphases complete.\n\nThis field is encoded as follows:"]
pub type PipetranslimitR = crate::FieldReader<Pipetranslimit>;
impl PipetranslimitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pipetranslimit> {
        match self.bits {
            0 => Some(Pipetranslimit::D0),
            1 => Some(Pipetranslimit::D1),
            2 => Some(Pipetranslimit::D2),
            3 => Some(Pipetranslimit::D3),
            _ => None,
        }
    }
    #[doc = "1 request"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Pipetranslimit::D0
    }
    #[doc = "2 requests"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Pipetranslimit::D1
    }
    #[doc = "3 requests"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Pipetranslimit::D2
    }
    #[doc = "4 requests ... F: 16 requests"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Pipetranslimit::D3
    }
}
#[doc = "Field `PIPETRANSLIMIT` writer - AXI Pipelined Transfers Burst Request Limit\n\nThe field controls the number of outstanding pipelined transfer\n\nrequests the AXI master pushes to the AXI slave. When the AXI\n\nmaster reaches this limit, it does not make any more requests on\n\nthe AXI ARADDR and AWADDR buses until the associated data\n\nphases complete.\n\nThis field is encoded as follows:"]
pub type PipetranslimitW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pipetranslimit>;
impl<'a, REG> PipetranslimitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 request"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipetranslimit::D0)
    }
    #[doc = "2 requests"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipetranslimit::D1)
    }
    #[doc = "3 requests"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Pipetranslimit::D2)
    }
    #[doc = "4 requests ... F: 16 requests"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Pipetranslimit::D3)
    }
}
#[doc = "Field `EN1KPAGE` reader - 1K Page Boundary Enable\n\nBy default (this bit is disabled) the AXI breaks transfers at the 4k\n\npage boundary. When this bit is enabled, the AXI master (DMA\n\ndata) breaks transfers at the 1k page boundary."]
pub type En1kpageR = crate::BitReader;
#[doc = "Field `EN1KPAGE` writer - 1K Page Boundary Enable\n\nBy default (this bit is disabled) the AXI breaks transfers at the 4k\n\npage boundary. When this bit is enabled, the AXI master (DMA\n\ndata) breaks transfers at the 1k page boundary."]
pub type En1kpageW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:11 - AXI Pipelined Transfers Burst Request Limit\n\nThe field controls the number of outstanding pipelined transfer\n\nrequests the AXI master pushes to the AXI slave. When the AXI\n\nmaster reaches this limit, it does not make any more requests on\n\nthe AXI ARADDR and AWADDR buses until the associated data\n\nphases complete.\n\nThis field is encoded as follows:"]
    #[inline(always)]
    pub fn pipetranslimit(&self) -> PipetranslimitR {
        PipetranslimitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 1K Page Boundary Enable\n\nBy default (this bit is disabled) the AXI breaks transfers at the 4k\n\npage boundary. When this bit is enabled, the AXI master (DMA\n\ndata) breaks transfers at the 1k page boundary."]
    #[inline(always)]
    pub fn en1kpage(&self) -> En1kpageR {
        En1kpageR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11 - AXI Pipelined Transfers Burst Request Limit\n\nThe field controls the number of outstanding pipelined transfer\n\nrequests the AXI master pushes to the AXI slave. When the AXI\n\nmaster reaches this limit, it does not make any more requests on\n\nthe AXI ARADDR and AWADDR buses until the associated data\n\nphases complete.\n\nThis field is encoded as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn pipetranslimit(&mut self) -> PipetranslimitW<Usb3Gsbuscfg1Spec> {
        PipetranslimitW::new(self, 8)
    }
    #[doc = "Bit 12 - 1K Page Boundary Enable\n\nBy default (this bit is disabled) the AXI breaks transfers at the 4k\n\npage boundary. When this bit is enabled, the AXI master (DMA\n\ndata) breaks transfers at the 1k page boundary."]
    #[inline(always)]
    #[must_use]
    pub fn en1kpage(&mut self) -> En1kpageW<Usb3Gsbuscfg1Spec> {
        En1kpageW::new(self, 12)
    }
}
#[doc = "Global SoC Bus Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gsbuscfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gsbuscfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3Gsbuscfg1Spec;
impl crate::RegisterSpec for Usb3Gsbuscfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gsbuscfg1::R`](R) reader structure"]
impl crate::Readable for Usb3Gsbuscfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gsbuscfg1::W`](W) writer structure"]
impl crate::Writable for Usb3Gsbuscfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GSBUSCFG1 to value 0x0300"]
impl crate::Resettable for Usb3Gsbuscfg1Spec {
    const RESET_VALUE: u32 = 0x0300;
}
