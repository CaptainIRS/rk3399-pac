#[doc = "Register `PCIE_LM_END_POINT_BUS_AND_DEVICE_NUMBER` reader"]
pub type R = crate::R<PcieLmEndPointBusAndDeviceNumberSpec>;
#[doc = "Field `EPDN` reader - Device Number \\[EPDN\\]
Device Number captured by Function 0 in End Point mode"]
pub type EpdnR = crate::FieldReader;
#[doc = "Field `R5` reader - Reserved \\[R5\\]
Reserved"]
pub type R5R = crate::FieldReader;
#[doc = "Field `EPBN` reader - Bus Number \\[EPBN\\]
Bus Number captured by Function 0 in End Point mode"]
pub type EpbnR = crate::FieldReader;
#[doc = "Field `R16` reader - Reserved \\[R16\\]
Reserved"]
pub type R16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - Device Number \\[EPDN\\]
Device Number captured by Function 0 in End Point mode"]
    #[inline(always)]
    pub fn epdn(&self) -> EpdnR {
        EpdnR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Reserved \\[R5\\]
Reserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Bus Number \\[EPBN\\]
Bus Number captured by Function 0 in End Point mode"]
    #[inline(always)]
    pub fn epbn(&self) -> EpbnR {
        EpbnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Reserved \\[R16\\]
Reserved"]
    #[inline(always)]
    pub fn r16(&self) -> R16R {
        R16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "End Point Bus and Device Number Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_end_point_bus_and_device_number::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmEndPointBusAndDeviceNumberSpec;
impl crate::RegisterSpec for PcieLmEndPointBusAndDeviceNumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_end_point_bus_and_device_number::R`](R) reader structure"]
impl crate::Readable for PcieLmEndPointBusAndDeviceNumberSpec {}
#[doc = "`reset()` method sets PCIE_LM_END_POINT_BUS_AND_DEVICE_NUMBER to value 0"]
impl crate::Resettable for PcieLmEndPointBusAndDeviceNumberSpec {
    const RESET_VALUE: u32 = 0;
}
