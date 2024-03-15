#[doc = "Register `DDRMON_IP_VERSION` reader"]
pub type R = crate::R<DdrmonIpVersionSpec>;
#[doc = "Field `IP_VERSION` reader - DDR monitor IP version"]
pub type IpVersionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - DDR monitor IP version"]
    #[inline(always)]
    pub fn ip_version(&self) -> IpVersionR {
        IpVersionR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DDR Monitor IP Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ip_version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonIpVersionSpec;
impl crate::RegisterSpec for DdrmonIpVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ip_version::R`](R) reader structure"]
impl crate::Readable for DdrmonIpVersionSpec {}
#[doc = "`reset()` method sets DDRMON_IP_VERSION to value 0"]
impl crate::Resettable for DdrmonIpVersionSpec {
    const RESET_VALUE: u32 = 0;
}
