#[doc = "Register `CEC_ADDR_H` reader"]
pub type R = crate::R<CecAddrHSpec>;
#[doc = "Register `CEC_ADDR_H` writer"]
pub type W = crate::W<CecAddrHSpec>;
#[doc = "Field `CEC_ADDR_H_0` reader - Logical address 8 - Playback Device 2"]
pub type CecAddrH0R = crate::BitReader;
#[doc = "Field `CEC_ADDR_H_0` writer - Logical address 8 - Playback Device 2"]
pub type CecAddrH0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_H_1` reader - Logical address 9 - Playback Device 3"]
pub type CecAddrH1R = crate::BitReader;
#[doc = "Field `CEC_ADDR_H_1` writer - Logical address 9 - Playback Device 3"]
pub type CecAddrH1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_H_2` reader - Logical address 10 - Tuner 4"]
pub type CecAddrH2R = crate::BitReader;
#[doc = "Field `CEC_ADDR_H_2` writer - Logical address 10 - Tuner 4"]
pub type CecAddrH2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_H_3` reader - Logical address 11 - Playback Device 3"]
pub type CecAddrH3R = crate::BitReader;
#[doc = "Field `CEC_ADDR_H_3` writer - Logical address 11 - Playback Device 3"]
pub type CecAddrH3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_H_4` reader - Logical address 12 - Reserved"]
pub type CecAddrH4R = crate::BitReader;
#[doc = "Field `CEC_ADDR_H_4` writer - Logical address 12 - Reserved"]
pub type CecAddrH4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_H_5` reader - Logical address 13 - Reserved"]
pub type CecAddrH5R = crate::BitReader;
#[doc = "Field `CEC_ADDR_H_5` writer - Logical address 13 - Reserved"]
pub type CecAddrH5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_H_6` reader - Logical address 14 - Free use"]
pub type CecAddrH6R = crate::BitReader;
#[doc = "Field `CEC_ADDR_H_6` writer - Logical address 14 - Free use"]
pub type CecAddrH6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_H_7` reader - Logical address 15 - Unregistered (as initiator\n\naddress), Broadcast (as destination address)"]
pub type CecAddrH7R = crate::BitReader;
#[doc = "Field `CEC_ADDR_H_7` writer - Logical address 15 - Unregistered (as initiator\n\naddress), Broadcast (as destination address)"]
pub type CecAddrH7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Logical address 8 - Playback Device 2"]
    #[inline(always)]
    pub fn cec_addr_h_0(&self) -> CecAddrH0R {
        CecAddrH0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical address 9 - Playback Device 3"]
    #[inline(always)]
    pub fn cec_addr_h_1(&self) -> CecAddrH1R {
        CecAddrH1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical address 10 - Tuner 4"]
    #[inline(always)]
    pub fn cec_addr_h_2(&self) -> CecAddrH2R {
        CecAddrH2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical address 11 - Playback Device 3"]
    #[inline(always)]
    pub fn cec_addr_h_3(&self) -> CecAddrH3R {
        CecAddrH3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical address 12 - Reserved"]
    #[inline(always)]
    pub fn cec_addr_h_4(&self) -> CecAddrH4R {
        CecAddrH4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical address 13 - Reserved"]
    #[inline(always)]
    pub fn cec_addr_h_5(&self) -> CecAddrH5R {
        CecAddrH5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical address 14 - Free use"]
    #[inline(always)]
    pub fn cec_addr_h_6(&self) -> CecAddrH6R {
        CecAddrH6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical address 15 - Unregistered (as initiator\n\naddress), Broadcast (as destination address)"]
    #[inline(always)]
    pub fn cec_addr_h_7(&self) -> CecAddrH7R {
        CecAddrH7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Logical address 8 - Playback Device 2"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_h_0(&mut self) -> CecAddrH0W<CecAddrHSpec> {
        CecAddrH0W::new(self, 0)
    }
    #[doc = "Bit 1 - Logical address 9 - Playback Device 3"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_h_1(&mut self) -> CecAddrH1W<CecAddrHSpec> {
        CecAddrH1W::new(self, 1)
    }
    #[doc = "Bit 2 - Logical address 10 - Tuner 4"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_h_2(&mut self) -> CecAddrH2W<CecAddrHSpec> {
        CecAddrH2W::new(self, 2)
    }
    #[doc = "Bit 3 - Logical address 11 - Playback Device 3"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_h_3(&mut self) -> CecAddrH3W<CecAddrHSpec> {
        CecAddrH3W::new(self, 3)
    }
    #[doc = "Bit 4 - Logical address 12 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_h_4(&mut self) -> CecAddrH4W<CecAddrHSpec> {
        CecAddrH4W::new(self, 4)
    }
    #[doc = "Bit 5 - Logical address 13 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_h_5(&mut self) -> CecAddrH5W<CecAddrHSpec> {
        CecAddrH5W::new(self, 5)
    }
    #[doc = "Bit 6 - Logical address 14 - Free use"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_h_6(&mut self) -> CecAddrH6W<CecAddrHSpec> {
        CecAddrH6W::new(self, 6)
    }
    #[doc = "Bit 7 - Logical address 15 - Unregistered (as initiator\n\naddress), Broadcast (as destination address)"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_h_7(&mut self) -> CecAddrH7W<CecAddrHSpec> {
        CecAddrH7W::new(self, 7)
    }
}
#[doc = "CEC Logical Address Register High\n\nThis register indicates the logical address(es) allocated to the CEC device.\n\nThis register is written by software when the logical allocation is finished. Bit value 1 means\n\nthe corresponding logical address is allocated to this device. Bit value 0 means the\n\ncorresponding logical address is not allocated to this device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_addr_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_addr_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecAddrHSpec;
impl crate::RegisterSpec for CecAddrHSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cec_addr_h::R`](R) reader structure"]
impl crate::Readable for CecAddrHSpec {}
#[doc = "`write(|w| ..)` method takes [`cec_addr_h::W`](W) writer structure"]
impl crate::Writable for CecAddrHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CEC_ADDR_H to value 0x80"]
impl crate::Resettable for CecAddrHSpec {
    const RESET_VALUE: u8 = 0x80;
}
