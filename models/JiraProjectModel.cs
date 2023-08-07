using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace Effectsoft.Atlassian.Jira.Models
{
    public class JiraProjectModel
    {
        // Root myDeserializedClass = JsonConvert.DeserializeObject<Root>(myJsonResponse);
        public class AvatarUrls
        {
            [JsonProperty("48x48")]
            public string? _48x48 { get; set; }

            [JsonProperty("24x24")]
            public string? _24x24 { get; set; }

            [JsonProperty("16x16")]
            public string? _16x16 { get; set; }

            [JsonProperty("32x32")]
            public string? _32x32 { get; set; }
        }

        public class Insight
        {
            public int totalIssueCount { get; set; }
            public DateTime lastIssueUpdateTime { get; set; }
        }

        public class ProjectCategory
        {
            public string? self { get; set; }
            public string? id { get; set; }
            public string? name { get; set; }
            public string? description { get; set; }
        }

        public class Root
        {
            public string? self { get; set; }
            public string? nextPage { get; set; }
            public int maxResults { get; set; }
            public int startAt { get; set; }
            public int total { get; set; }
            public bool isLast { get; set; }
            public List<Value> values { get; set; }
        }

        public class Value
        {
            public string? self { get; set; }
            public string? id { get; set; }
            public string? key { get; set; }
            public string? name { get; set; }
            public AvatarUrls avatarUrls { get; set; }
            public ProjectCategory projectCategory { get; set; }
            public bool simplified { get; set; }
            public string? style { get; set; }
            public Insight insight { get; set; }
        }


        public class Version
        {
            public string? self { get; set; }
            public string? id { get; set; }
            public string? description { get; set; }
            public string? name { get; set; }
            public bool archived { get; set; }
            public bool released { get; set; }
            public string? releaseDate { get; set; }
            public bool overdue { get; set; }
            public string? userReleaseDate { get; set; }
            public string? projectId { get; set; }
        }



    }
}
