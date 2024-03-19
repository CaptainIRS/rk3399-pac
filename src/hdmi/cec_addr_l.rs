#[doc = "Register `CEC_ADDR_L` reader"]
pub type R = crate::R<CecAddrLSpec>;
#[doc = "Register `CEC_ADDR_L` writer"]
pub type W = crate::W<CecAddrLSpec>;
#[doc = "Field `CEC_ADDR_L_0` reader - Logical address 0 - Device TV"]
pub type CecAddrL0R = crate::BitReader;
#[doc = "Field `CEC_ADDR_L_0` writer - Logical address 0 - Device TV"]
pub type CecAddrL0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_L_1` reader - Logical address 1 - Recording Device 1"]
pub type CecAddrL1R = crate::BitReader;
#[doc = "Field `CEC_ADDR_L_1` writer - Logical address 1 - Recording Device 1"]
pub type CecAddrL1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_L_2` reader - Logical address 2 - Recording Device 2"]
pub type CecAddrL2R = crate::BitReader;
#[doc = "Field `CEC_ADDR_L_2` writer - Logical address 2 - Recording Device 2"]
pub type CecAddrL2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_L_3` reader - Logical address 3 - Tuner 1"]
pub type CecAddrL3R = crate::BitReader;
#[doc = "Field `CEC_ADDR_L_3` writer - Logical address 3 - Tuner 1"]
pub type CecAddrL3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_L_4` reader - Logical address 4 - Playback Device 1"]
pub type CecAddrL4R = crate::BitReader;
#[doc = "Field `CEC_ADDR_L_4` writer - Logical address 4 - Playback Device 1"]
pub type CecAddrL4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_L_5` reader - Logical address 5 - Audio System"]
pub type CecAddrL5R = crate::BitReader;
#[doc = "Field `CEC_ADDR_L_5` writer - Logical address 5 - Audio System"]
pub type CecAddrL5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_L_6` reader - Logical address 6 - Tuner 2"]
pub type CecAddrL6R = crate::BitReader;
#[doc = "Field `CEC_ADDR_L_6` writer - Logical address 6 - Tuner 2"]
pub type CecAddrL6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEC_ADDR_L_7` reader - Logical address 7 - Tuner 3"]
pub type CecAddrL7R = crate::BitReader;
#[doc = "Field `CEC_ADDR_L_7` writer - Logical address 7 - Tuner 3"]
pub type CecAddrL7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Logical address 0 - Device TV"]
    #[inline(always)]
    pub fn cec_addr_l_0(&self) -> CecAddrL0R {
        CecAddrL0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical address 1 - Recording Device 1"]
    #[inline(always)]
    pub fn cec_addr_l_1(&self) -> CecAddrL1R {
        CecAddrL1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical address 2 - Recording Device 2"]
    #[inline(always)]
    pub fn cec_addr_l_2(&self) -> CecAddrL2R {
        CecAddrL2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical address 3 - Tuner 1"]
    #[inline(always)]
    pub fn cec_addr_l_3(&self) -> CecAddrL3R {
        CecAddrL3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical address 4 - Playback Device 1"]
    #[inline(always)]
    pub fn cec_addr_l_4(&self) -> CecAddrL4R {
        CecAddrL4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical address 5 - Audio System"]
    #[inline(always)]
    pub fn cec_addr_l_5(&self) -> CecAddrL5R {
        CecAddrL5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical address 6 - Tuner 2"]
    #[inline(always)]
    pub fn cec_addr_l_6(&self) -> CecAddrL6R {
        CecAddrL6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical address 7 - Tuner 3"]
    #[inline(always)]
    pub fn cec_addr_l_7(&self) -> CecAddrL7R {
        CecAddrL7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Logical address 0 - Device TV"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_l_0(&mut self) -> CecAddrL0W<CecAddrLSpec> {
        CecAddrL0W::new(self, 0)
    }
    #[doc = "Bit 1 - Logical address 1 - Recording Device 1"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_l_1(&mut self) -> CecAddrL1W<CecAddrLSpec> {
        CecAddrL1W::new(self, 1)
    }
    #[doc = "Bit 2 - Logical address 2 - Recording Device 2"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_l_2(&mut self) -> CecAddrL2W<CecAddrLSpec> {
        CecAddrL2W::new(self, 2)
    }
    #[doc = "Bit 3 - Logical address 3 - Tuner 1"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_l_3(&mut self) -> CecAddrL3W<CecAddrLSpec> {
        CecAddrL3W::new(self, 3)
    }
    #[doc = "Bit 4 - Logical address 4 - Playback Device 1"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_l_4(&mut self) -> CecAddrL4W<CecAddrLSpec> {
        CecAddrL4W::new(self, 4)
    }
    #[doc = "Bit 5 - Logical address 5 - Audio System"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_l_5(&mut self) -> CecAddrL5W<CecAddrLSpec> {
        CecAddrL5W::new(self, 5)
    }
    #[doc = "Bit 6 - Logical address 6 - Tuner 2"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_l_6(&mut self) -> CecAddrL6W<CecAddrLSpec> {
        CecAddrL6W::new(self, 6)
    }
    #[doc = "Bit 7 - Logical address 7 - Tuner 3"]
    #[inline(always)]
    #[must_use]
    pub fn cec_addr_l_7(&mut self) -> CecAddrL7W<CecAddrLSpec> {
        CecAddrL7W::new(self, 7)
    }
}
#[doc = "CEC Logical Address Register Low\n\nThis register indicates the logical address(es) allocated to the CEC device.\n\nThis register is written by software when the logical allocation is finished. Bit value 1 means\n\nthe corresponding logical address is allocated to this device. Bit value 0 means the\n\ncorresponding logical address is not allocated to this device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_addr_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_addr_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecAddrLSpec;
impl crate::RegisterSpec for CecAddrLSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cec_addr_l::R`](R) reader structure"]
impl crate::Readable for CecAddrLSpec {}
#[doc = "`write(|w| ..)` method takes [`cec_addr_l::W`](W) writer structure"]
impl crate::Writable for CecAddrLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CEC_ADDR_L to value 0"]
impl crate::Resettable for CecAddrLSpec {
    const RESET_VALUE: u8 = 0;
}
